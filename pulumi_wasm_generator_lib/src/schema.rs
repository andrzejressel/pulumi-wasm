use crate::model::{ElementId, GlobalType, GlobalTypeProperty, InputProperty, OutputProperty, Ref};
use anyhow::{anyhow, Context, Result};
use serde::Deserialize;
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};

type PulumiMap<T> = BTreeMap<String, T>;

#[derive(Deserialize, Debug)]
pub(crate) enum TypeEnum {
    #[serde(alias = "boolean")]
    Boolean,
    #[serde(alias = "integer")]
    Integer,
    #[serde(alias = "number")]
    Number,
    #[serde(alias = "string")]
    String,
    #[serde(alias = "array")]
    Array,
    #[serde(alias = "object")]
    Object,
}

#[derive(Deserialize, Debug)]
struct Type {
    #[serde(rename = "type")]
    type_: Option<TypeEnum>,
    description: Option<String>,
    #[serde(rename = "$ref")]
    ref_: Option<String>,
    items: Option<Box<Type>>,
    #[serde(rename = "additionalProperties")]
    additional_properties: Option<Box<Type>>,
    #[serde(rename = "oneOf")]
    one_of: Option<Vec<OneOfType>>,
    discriminator: Option<Discriminator>,
}

#[derive(Deserialize, Debug)]
struct OneOfType {
    #[serde(rename = "$ref")]
    ref_: String,
}

#[derive(Deserialize, Debug)]
struct Discriminator {
    #[serde(rename = "propertyName")]
    property_name: String,
    mapping: BTreeMap<String, String>,
}

#[derive(Deserialize, Debug)]
struct Property {
    #[serde(flatten)]
    r#type: Type,
}

#[derive(Deserialize, Debug)]
struct ObjectType {
    description: Option<String>,
    r#type: Option<TypeEnum>,
    #[serde(default)]
    properties: PulumiMap<Property>,
    #[serde(default)]
    required: BTreeSet<String>,
}

#[derive(Deserialize, Debug)]
struct Resource {
    #[serde(flatten)]
    object_type: ObjectType,
    #[serde(default, rename = "inputProperties")]
    input_properties: PulumiMap<Property>,
    #[serde(default, rename = "requiredInputs")]
    required_inputs: BTreeSet<String>,
}

#[derive(Deserialize, Debug)]
struct EnumValue {
    name: Option<String>,
    description: Option<String>,
    // value: Option<String>, //apparently any
}

#[derive(Deserialize, Debug)]
struct ComplexType {
    #[serde(flatten)]
    object_type: ObjectType,
    #[serde(default)]
    r#enum: Vec<EnumValue>,
}

#[derive(Deserialize, Debug)]
struct Function {
    description: Option<String>,
    inputs: Option<ObjectType>,
    outputs: Option<ObjectType>,
}

#[derive(Deserialize, Debug)]
pub(crate) struct Package {
    name: String,
    #[serde(rename = "displayName")]
    display_name: Option<String>,
    #[serde(default)]
    resources: PulumiMap<Resource>,
    version: String,
    #[serde(default)]
    types: PulumiMap<ComplexType>,
    #[serde(default)]
    functions: PulumiMap<Function>,
}

// fn complex_type_mapper(complex_type: ComplexType) -> Result<crate::model::Type> {
//     //TODO: Enums
//     object_type_mapper(complex_type.object_type)
// }
//
// fn object_type_mapper(object_type: ObjectType) -> Result<crate::model::Type> {
//
//
// }

//TODO: Fix formatting
fn new_type_mapper(type_: &Type) -> Result<crate::model::Type> {
    (match type_ {
        Type {
            ref_: Some(ref r), ..
        } => Ok(crate::model::Type::Ref(
            Ref::new(r).context(format!("Cannot convert ref fo type {type_:?}"))?,
        )),
        Type {
            type_: Some(TypeEnum::Boolean),
            ..
        } => Ok(crate::model::Type::Boolean),
        Type {
            type_: Some(TypeEnum::Integer),
            ..
        } => Ok(crate::model::Type::Integer),
        Type {
            type_: Some(TypeEnum::Number),
            ..
        } => Ok(crate::model::Type::Number),
        Type {
            type_: Some(TypeEnum::String),
            ..
        } => Ok(crate::model::Type::String),
        Type {
            type_: Some(TypeEnum::Array),
            items: Some(items),
            ..
        } => Ok(crate::model::Type::Array(Box::new(new_type_mapper(items)?))),
        Type {
            type_: Some(TypeEnum::Array),
            ..
        } => Err(anyhow!("Array does not have 'items' field")),
        Type {
            type_: Some(TypeEnum::Object),
            additional_properties: Some(property),
            ..
        } => Ok(crate::model::Type::Object(Box::new(new_type_mapper(
            property,
        )?))),
        Type {
            type_: Some(TypeEnum::Object),
            ..
        } => Err(anyhow!("Object does not have 'additionalProperties' field")),
        Type {
            one_of: Some(one_of),
            discriminator: Some(discriminator),
            ..
        } => {
            create_discriminated_union(one_of, discriminator)
        },
        Type {
            one_of: Some(_),
            discriminator: None,
            ..
        } => Err(anyhow!(
            "Discriminated unions without discriminator are not supported"
        )),
        Type {
            type_: None,
            ref_: None,
            ..
        } => Err(anyhow!("'type' and 'ref' fields cannot be empty")),
    })
    .context(format!("Cannot handle type: [{type_:?}]"))
}

fn create_discriminated_union(one_of: &Vec<OneOfType>, discriminator: &Discriminator) -> Result<crate::model::Type> {

    let dest_to_discriminator: HashMap<_, _> = discriminator.mapping
        .iter()
        .map(|(k, v)| (v, k))
        .collect();


    Ok(
        crate::model::Type::DiscriminatedUnion(
            discriminator.property_name.clone(),
            one_of.iter().map(|r| {
                let discriminator = dest_to_discriminator[&r.ref_];
                (discriminator.clone(), Ref::new(&*r.ref_).context(format!("Cannot convert ref fo type {r:?}")).unwrap())
            })
                .collect()
        )
    )

    // panic!()
    // panic!()
}

fn resource_to_model(
    resource_name: &str,
    resource: &Resource,
) -> Result<(ElementId, crate::model::Resource)> {
    let element_id = ElementId::new(resource_name)?;
    Ok((
        element_id.clone(),
        crate::model::Resource {
            element_id: element_id.clone(),
            // name: resource_name.clone(),
            description: resource.object_type.description.clone(),
            input_properties: resource
                .input_properties
                .iter()
                .map(|(input_name, input_property)| {
                    let mut type_ = new_type_mapper(&input_property.r#type)
                        .context(format!("Cannot handle [{input_name}] type"))?;
                    // Forced options are not for inputs
                    if !resource.required_inputs.contains(input_name) {
                        type_ = crate::model::Type::Option(Box::new(type_));
                    }
                    Ok(crate::model::InputProperty {
                        name: input_name.clone(),
                        r#type: type_,
                        description: input_property.r#type.description.clone(),
                    })
                })
                .collect::<Result<Vec<_>>>()?,
            output_properties: convert_output_property_object_type(
                &element_id,
                &resource.object_type,
            )?,
        },
    ))
}

fn function_to_model(
    function_name: &str,
    function: &Function,
) -> Result<(ElementId, crate::model::Function)> {
    let element_id = ElementId::new(function_name)?;
    Ok((
        element_id.clone(),
        crate::model::Function {
            element_id: element_id.clone(),
            description: function.description.clone(),
            input_properties: match &function.inputs {
                None => vec![],
                Some(input) => convert_input_property_object_type(input)?,
            },
            output_properties: match &function.outputs {
                None => vec![],
                Some(output) => convert_output_property_object_type(&element_id, output)?,
            },
        },
    ))
}

fn convert_input_property_object_type(object_type: &ObjectType) -> Result<Vec<InputProperty>> {
    object_type
        .properties
        .iter()
        .map(|(output_name, output_property)| {
            let mut type_ = new_type_mapper(&output_property.r#type)
                .context(format!("Cannot handle [{output_name}] type"))?;
            if !object_type.required.contains(output_name) {
                type_ = crate::model::Type::Option(Box::new(type_));
            }
            Ok(crate::model::InputProperty {
                name: output_name.clone(),
                r#type: type_,
                description: output_property.r#type.description.clone(),
            })
        })
        .collect::<Result<Vec<_>>>()
}

fn convert_output_property_object_type(
    element_id: &ElementId,
    object_type: &ObjectType,
) -> Result<Vec<OutputProperty>> {
    let forced_options = invalid_required_complextype_required_fields();
    object_type
        .properties
        .iter()
        .map(|(output_name, output_property)| {
            let mut type_ = new_type_mapper(&output_property.r#type)
                .context(format!("Cannot handle [{output_name}] type"))?;
            if !object_type.required.contains(output_name)
                || forced_options.contains(&(element_id.clone(), output_name.clone()))
            {
                type_ = crate::model::Type::Option(Box::new(type_));
            }
            Ok(crate::model::OutputProperty {
                name: output_name.clone(),
                r#type: type_,
                description: output_property.r#type.description.clone(),
            })
        })
        .collect::<Result<Vec<_>>>()
}

pub(crate) fn to_model(package: &Package) -> Result<crate::model::Package> {
    let resources = package
        .resources
        .iter()
        .map(|(resource_name, resource)| resource_to_model(resource_name, resource))
        .collect::<Result<BTreeMap<_, _>>>()
        .context("Cannot handle resources")?;

    let functions = package
        .functions
        .iter()
        .map(|(function_name, function)| function_to_model(function_name, function))
        .collect::<Result<BTreeMap<_, _>>>()
        .context("Cannot handle functions")?;

    let types = package
        .types
        .iter()
        .map(|(type_name, type_)| {
            //TODO: Enums, support non objects
            let element_id = ElementId::new(type_name)?;
            let tpe = match type_.object_type {
                ObjectType { r#type: None, .. } => Err(anyhow!("Unknown complex type")),
                ObjectType {
                    r#type: Some(TypeEnum::Object),
                    ..
                } => Ok(GlobalType::Object(
                    convert_output_property_object_type(&element_id, &type_.object_type)?
                        .iter()
                        .map(|p| GlobalTypeProperty {
                            name: p.name.clone(),
                            r#type: p.r#type.clone(),
                            description: p.description.clone(),
                        })
                        .collect(),
                )),
                ObjectType {
                    r#type: Some(TypeEnum::Array),
                    ..
                } => Err(anyhow!("Array not supported")),
                ObjectType {
                    r#type: Some(TypeEnum::Boolean),
                    ..
                } => Ok(GlobalType::Boolean),
                ObjectType {
                    r#type: Some(TypeEnum::Integer),
                    ..
                } => Ok(GlobalType::Integer),
                ObjectType {
                    r#type: Some(TypeEnum::Number),
                    ..
                } => Ok(GlobalType::Number),
                ObjectType {
                    r#type: Some(TypeEnum::String),
                    ..
                } => Ok(GlobalType::String),
            }
            .context(format!("Cannot convert type [{type_name}]"))?;
            Ok((element_id, tpe))
        })
        .collect::<Result<BTreeMap<_, _>>>()
        .context("Cannot handle types")?;
    Ok(crate::model::Package {
        name: package.name.clone(),
        version: package.version.clone(),
        display_name: package.display_name.clone(),
        types,
        resources,
        functions,
    })
}

fn invalid_required_complextype_required_fields() -> HashSet<(ElementId, String)> {
    HashSet::from([
        // https://github.com/pulumi/pulumi-docker/issues/1052
        (
            ElementId::new("docker:index/container:Container").unwrap(),
            "containerLogs".to_string(),
        ),
        (
            ElementId::new("docker:index/container:Container").unwrap(),
            "healthcheck".to_string(),
        ),
        //
    ])
}

#[cfg(test)]
mod test {
    use crate::schema::to_model;
    use anyhow::Result;
    use serde_json::json;

    #[test]
    fn resource_with_invalid_id_fails() -> Result<()> {
        let json = json!({
            "name": "test",
            "version": "0.0.0",
            "resources": {
                "invalid": {
                    "description": "test resource",
                }
            }
        });

        let err = to_model(&serde_json::from_value(json)?).unwrap_err();

        let chain: Vec<_> = anyhow::Chain::new(err.as_ref())
            .map(|e| e.to_string())
            .collect();

        assert_eq!(
            vec![
                "Cannot handle resources",
                "Cannot generate element id from [invalid]",
            ],
            chain
        );

        Ok(())

        // assert!(err
        //     .to_string()
        //     .contains("Cannot generate element id from [invalid]"));
        //
        // Ok(())
    }

    #[test]
    fn object_without_additionalproperties_fails() -> Result<()> {
        let json = json!({
            "name": "test",
            "version": "0.0.0-DEV",
            "resources": {
                "test:index:test_resource": {
                    "description": "test resource",
                    "inputProperties": {
                        "test_input": {
                            "type": "object"
                        }
                    },
                }
            }
        });

        let err = to_model(&serde_json::from_value(json)?).unwrap_err();

        let chain: Vec<_> = anyhow::Chain::new(err.as_ref())
            .map(|e| e.to_string())
            .collect();

        assert_eq!(
            vec![
                "Cannot handle resources",
                "Cannot handle [test_input] type",
                "Cannot handle type: [Type { type_: Some(Object), description: None, ref_: None, items: None, additional_properties: None }]",
                "Object does not have 'additionalProperties' field",
            ],
            chain
        );

        Ok(())
    }

    #[test]
    fn array_without_items_fails() -> Result<()> {
        let json = json!({
            "name": "test",
            "version": "0.0.0-DEV",
            "resources": {
                "test:index:test_resource": {
                    "description": "test resource",
                    "inputProperties": {
                        "test_input": {
                            "type": "array"
                        }
                    },
                }
            }
        });

        let err = to_model(&serde_json::from_value(json)?).unwrap_err();

        let chain: Vec<_> = anyhow::Chain::new(err.as_ref())
            .map(|e| e.to_string())
            .collect();

        assert_eq!(
            vec![
                "Cannot handle resources",
                "Cannot handle [test_input] type",
                "Cannot handle type: [Type { type_: Some(Array), description: None, ref_: None, items: None, additional_properties: None }]",
                "Array does not have 'items' field",
            ],
            chain
        );

        Ok(())
    }
}
