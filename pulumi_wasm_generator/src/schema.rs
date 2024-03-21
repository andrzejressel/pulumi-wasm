use std::collections::HashMap;

use serde::Deserialize;

type PulumiMap<T> = HashMap<String, T>;

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
    required: Vec<String>,
}

#[derive(Deserialize, Debug)]
struct Resource {
    #[serde(flatten)]
    object_type: ObjectType,
    #[serde(default)]
    input_properties: PulumiMap<Property>,
    #[serde(default, rename = "requiredInputs")]
    required_inputs: Vec<String>,
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
    r#enum: Vec<EnumValue>
}

#[derive(Deserialize, Debug)]
pub(crate) struct Package {
    name: String,
    #[serde(rename = "displayName")]
    display_name: Option<String>,
    #[serde(default)]
    resources: PulumiMap<Resource>,
    #[serde(default)]
    types: PulumiMap<ComplexType>,
}
