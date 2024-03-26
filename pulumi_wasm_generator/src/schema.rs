use std::collections::{BTreeMap, BTreeSet};

use serde::Deserialize;
use crate::model::{ElementId, InputProperty, OutputProperty};
use anyhow::Result;

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

fn resource_to_model(resource_name: &String, resource: &Resource) -> Result<(ElementId, crate::model::Resource)> {
    let element_id = ElementId::new(resource_name)?;
    return Ok((element_id, crate::model::Resource {
        // name: resource_name.clone(),
        description: resource.object_type.description.clone(),
        input_properties: resource.input_properties.iter().map(|(input_name, input_property)| {
            return crate::model::InputProperty {
                name: input_name.clone(),
                // r#type: match &input_property.r#type.r#type {
                //     Some(t) => crate::model::TypeOrRef::Type(match t),
                //     None => crate::model::TypeOrRef::Ref(input_property.r#type.r#ref.clone().unwrap()),
                // },
                // description: input_property.descriptio
                required: resource.required_inputs.contains(input_name),
            };
        }).collect(),
        output_properties: resource.object_type.properties.iter().map(|(output_name, output_property)| {
            return crate::model::OutputProperty {
                name: output_name.clone(),
                // r#type: match &output_property.r#type.r#type {
                //     Some(t) => crate::model::TypeOrRef::Type(match t),
                //     None => crate::model::TypeOrRef::Ref(output_property.r#type.r#ref.clone().unwrap()),
                // },
                // description: output_property.description.clone(),
                required: resource.object_type.required.contains(output_name),
            };
        }).collect(),
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
        resources: resources,
    });
}