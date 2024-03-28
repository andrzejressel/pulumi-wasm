use std::collections::{BTreeMap, BTreeSet};

use serde::Deserialize;
use crate::model::ElementId;
use anyhow::{anyhow, Result};

type PulumiMap<T> = BTreeMap<String, T>;

#[derive(Deserialize, Debug)]
pub(crate) enum TypeType {
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
    r#type: Option<TypeType>,
    #[serde(rename = "$ref")]
    r#ref: Option<String>,
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

fn type_type_to_model(type_type: &TypeType) -> Result<crate::model::TypeType> {
    return match type_type {
        TypeType::Boolean => Ok(crate::model::TypeType::Boolean),
        TypeType::Integer => Ok(crate::model::TypeType::Integer),
        TypeType::Number => Ok(crate::model::TypeType::Number),
        TypeType::String => Ok(crate::model::TypeType::String),
        TypeType::Array => Ok(crate::model::TypeType::Array),
        TypeType::Object => Ok(crate::model::TypeType::Object),
    };
}

fn type_to_model(type_: &Type) -> Result<crate::model::TypeOrRef> {
    return match (&type_.r#type, &type_.r#ref) {
        (Some(_), Some(_)) => return Err(anyhow!("Cannot have both type and ref in a type. [{type_:?}]")),
        (None, None) => return Err(anyhow!("Must have either type or ref in a type. [{type_:?}]")),
        (Some(t), None) => Ok(crate::model::TypeOrRef::Type(type_type_to_model(t)?)),
        (None, Some(ref_)) => Ok(crate::model::TypeOrRef::Ref(ref_.clone())),
    }
    // return match &type_.r#type {
    //     Some(t) => crate::model::TypeOrRef::Type(match t),
    //     None => crate::model::TypeOrRef::Ref(type_.r#ref.clone().unwrap()),
    // };
}

fn resource_to_model(resource_name: &String, resource: &Resource) -> Result<(ElementId, crate::model::Resource)> {
    let element_id = ElementId::new(resource_name)?;
    return Ok((element_id, crate::model::Resource {
        // name: resource_name.clone(),
        description: resource.object_type.description.clone(),
        input_properties: resource.input_properties.iter().map(|(input_name, input_property)| {
            return Ok(crate::model::InputProperty {
                name: input_name.clone(),
                r#type: type_to_model(&input_property.r#type)?,
                // r#type: match &input_property.r#type.r#type {
                //     Some(t) => crate::model::TypeOrRef::Type(match t),
                //     None => crate::model::TypeOrRef::Ref(input_property.r#type.r#ref.clone().unwrap()),
                // },
                // description: input_property.descriptio
                required: resource.required_inputs.contains(input_name),
            });
        }).collect::<Result<Vec<_>>>()?,
        output_properties: resource.object_type.properties.iter().map(|(output_name, output_property)| {
            return Ok(crate::model::OutputProperty {
                name: output_name.clone(),
                r#type: type_to_model(&output_property.r#type)?,
                // r#type: match &output_property.r#type.r#type {
                //     Some(t) => crate::model::TypeOrRef::Type(match t),
                //     None => crate::model::TypeOrRef::Ref(output_property.r#type.r#ref.clone().unwrap()),
                // },
                // description: output_property.description.clone(),
                required: resource.object_type.required.contains(output_name),
            });
        }).collect::<Result<Vec<_>>>()?,
    }));
}

// fn create_output_properties(resource: &Resource) -> Result<BTreeMap<ElementId, OutputProperty>> {
//     let map = resource.object_type.properties.iter().map(|(output_name, output_property)| {
//         let element_id = ElementId::new(output_name)?;
//         Ok((element_id, OutputProperty {
//             // name: output_name.clone(),
//             // r#type: match &output_property.r#type.r#type {
//             //     Some(t) => crate::model::TypeOrRef::Type(match t),
//             //     None => crate::model::TypeOrRef::Ref(output_property.r#type.r#ref.clone().unwrap()),
//             // },
//             // description: output_property.description.clone(),
//             required: resource.object_type.required.contains(output_name),
//         }))
//     }).collect::<Result<_>>()?;
//     Ok(map)
// }
//
// fn create_input_properties_map(resource: &Resource) -> Result<BTreeMap<ElementId, InputProperty>> {
//     let map: BTreeMap<ElementId, InputProperty> = resource.input_properties.iter().map(|(input_name, input_property)| {
//         let element_id = ElementId::new(input_name)?;
//         Ok((element_id, InputProperty {
//             // name: input_name.clone(),
//             // r#type: match &input_property.r#type.r#type {
//             //     Some(t) => crate::model::TypeOrRef::Type(match t),
//             //     None => crate::model::TypeOrRef::Ref(input_property.r#type.r#ref.clone().unwrap()),
//             // },
//             // description: input_property.descriptio
//             required: resource.required_inputs.contains(input_name),
//         }))
//     }).collect::<Result<_>>()?;
//     Ok(map)
// }

pub(crate) fn to_model(package: &Package) -> Result<crate::model::Package> {
    let resources = package.resources.iter().map(|(resource_name, resource)| {
        resource_to_model(resource_name, resource)
    }).collect::<Result<BTreeMap<_, _>>>()?;
    return Ok(crate::model::Package {
        name: package.name.clone(),
        version: package.version.clone(),
        display_name: package.display_name.clone(),
        resources,
    });
}

#[cfg(test)]
mod test {
    use serde_json::json;
    use anyhow::Result;
    use crate::schema::to_model;

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
        assert!(err.to_string().contains("Cannot generate element id from [invalid]"));

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
        assert!(err.to_string().contains("Cannot generate element id from [invalid]"));

        Ok(())
    }

}