use std::collections::{BTreeMap, BTreeSet};

use crate::model::ElementId;
use anyhow::{anyhow, Context, Result};
use serde::Deserialize;

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
    #[serde(rename = "$ref")]
    ref_: Option<String>,
    items: Option<Box<Type>>,
    #[serde(rename = "additionalProperties")]
    additional_properties: Option<Box<Type>>,
}

#[derive(Deserialize, Debug)]
struct Property {
    #[serde(flatten)]
    r#type: Type,
}

#[derive(Deserialize, Debug)]
struct ObjectType {
    description: Option<String>,
    r#type: Option<String>,
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
    #[serde(default)]
    required: BTreeSet<String>,
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
pub(crate) struct Package {
    name: String,
    #[serde(rename = "displayName")]
    display_name: Option<String>,
    #[serde(default)]
    resources: PulumiMap<Resource>,
    version: String,
    #[serde(default)]
    types: PulumiMap<ComplexType>,
}

//TODO: Fix formatting
fn new_type_mapper(type_: &Type) -> Result<crate::model::Type> {
    (match type_ {
        Type {
            ref_: Some(ref r), ..
        } => Ok(crate::model::Type::Ref(r.to_string())),
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
            type_: None,
            ref_: None,
            ..
        } => Err(anyhow!("'type' and 'ref' fields cannot be empty")),
    })
    .context(format!("Cannot handle type: [{type_:?}]"))
}

fn resource_to_model(
    resource_name: &str,
    resource: &Resource,
) -> Result<(ElementId, crate::model::Resource)> {
    let element_id = ElementId::new(resource_name)?;
    Ok((
        element_id,
        crate::model::Resource {
            // name: resource_name.clone(),
            description: resource.object_type.description.clone(),
            input_properties: resource
                .input_properties
                .iter()
                .map(|(input_name, input_property)| {
                    let mut type_ = new_type_mapper(&input_property.r#type)
                        .context(format!("Cannot handle [{input_name}] type"))?;
                    if !resource.required_inputs.contains(input_name) {
                        type_ = crate::model::Type::Option(Box::new(type_));
                    }
                    Ok(crate::model::InputProperty {
                        name: input_name.clone(),
                        r#type: type_,
                    })
                })
                .collect::<Result<Vec<_>>>()?,
            output_properties: resource
                .object_type
                .properties
                .iter()
                .map(|(output_name, output_property)| {
                    let mut type_ = new_type_mapper(&output_property.r#type)
                        .context(format!("Cannot handle [{output_name}] type"))?;
                    if !resource.required.contains(output_name) {
                        type_ = crate::model::Type::Option(Box::new(type_));
                    }
                    Ok(crate::model::OutputProperty {
                        name: output_name.clone(),
                        r#type: type_,
                    })
                })
                .collect::<Result<Vec<_>>>()?,
        },
    ))
}

pub(crate) fn to_model(package: &Package) -> Result<crate::model::Package> {
    let resources = package
        .resources
        .iter()
        .map(|(resource_name, resource)| resource_to_model(resource_name, resource))
        .collect::<Result<BTreeMap<_, _>>>()?;
    Ok(crate::model::Package {
        name: package.name.clone(),
        version: package.version.clone(),
        display_name: package.display_name.clone(),
        resources,
    })
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
        assert!(err
            .to_string()
            .contains("Cannot generate element id from [invalid]"));

        Ok(())
    }

    #[test]
    fn object_without_additionalproperties_fails() -> Result<()> {
        let json = json!({
            "name": "test",
            "version": "0.0.0",
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
                "Cannot handle [test_input] type",
                "Cannot handle type: [Type { type_: Some(Object), ref_: None, items: None, additional_properties: None }]",
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
            "version": "0.0.0",
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
                "Cannot handle [test_input] type",
                "Cannot handle type: [Type { type_: Some(Array), ref_: None, items: None, additional_properties: None }]",
                "Array does not have 'items' field",
            ],
            chain
        );

        Ok(())
    }
}
