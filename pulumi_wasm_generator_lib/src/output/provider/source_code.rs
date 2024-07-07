use crate::model::{ElementId, GlobalType, GlobalTypeProperty, Ref, Type};
use crate::output::replace_multiple_dashes;
use anyhow::Context;
use handlebars::Handlebars;
use msgpack_protobuf_converter::Type as ConverterType;
use std::collections::BTreeMap;

use serde::Serialize;
use serde_json::json;

static TEMPLATE: &str = include_str!("lib.rs.handlebars");

#[derive(Serialize)]
struct InputProperty {
    name: String,
    arg_name: String,
}

#[derive(Serialize)]
struct OutputProperty {
    name: String,
    arg_name: String,
    required: bool,
    schema: Vec<u8>,
}

#[derive(Serialize)]
struct Interface {
    name: String,
    r#type: String,
    input_properties: Vec<InputProperty>,
    output_properties: Vec<OutputProperty>,
}

#[derive(Serialize)]
struct Package {
    name: String,
    interfaces: Vec<Interface>,
}

fn convert_model(package: &crate::model::Package) -> Package {
    Package {
        name: package.name.clone(),
        interfaces: package
            .resources
            .iter()
            .map(|(element_id, resource)| Interface {
                name: create_valid_element_id(element_id),
                r#type: element_id.raw.clone(),
                input_properties: resource
                    .input_properties
                    .iter()
                    .map(|input_property| InputProperty {
                        name: input_property.name.clone(),
                        arg_name: create_valid_id(&input_property.name),
                    })
                    .collect(),
                output_properties: resource
                    .output_properties
                    .iter()
                    .map(|output_property| OutputProperty {
                        name: output_property.name.clone(),
                        arg_name: create_valid_id(&output_property.name),
                        required: !matches!(output_property.r#type, Type::Option(_)),
                        schema: rmp_serde::encode::to_vec(&generate_schema(
                            &output_property.r#type,
                            &package.types,
                        ))
                        .context(format!(
                            "Cannot convert schema for {}",
                            output_property.name
                        ))
                        .unwrap(),
                    })
                    .collect(),
            })
            .collect(),
    }
}

fn create_valid_element_id(element_id: &ElementId) -> String {
    let mut vec = element_id.namespace.clone();
    vec.push(element_id.name.clone());
    create_valid_id(&vec.join("-"))
}

fn create_valid_id(s: &str) -> String {
    let result: String = s
        .chars()
        .map(|c| {
            if c.is_uppercase() {
                format!("-{}", c.to_lowercase())
            } else if !c.is_alphanumeric() {
                "-".to_string()
            } else {
                c.to_string()
            }
        })
        .collect();

    let result = replace_multiple_dashes(&result);
    let result = result.trim_matches('-').to_string();

    result.replace('-', "_")
}

pub(crate) fn generate_source_code(package: &crate::model::Package) -> String {
    let handlebars = Handlebars::new();
    handlebars
        .render_template(TEMPLATE, &json!({"package": &convert_model(package)}))
        .unwrap()
}

fn generate_schema(t: &Type, all_types: &BTreeMap<ElementId, GlobalType>) -> ConverterType {
    match t {
        Type::Boolean => ConverterType::Bool,
        Type::Integer => ConverterType::Int,
        Type::Number => ConverterType::Double,
        Type::String => ConverterType::String,
        Type::Array(t) => ConverterType::Array(Box::from(generate_schema(t, all_types))),
        Type::Object(t) => {
            ConverterType::SingleTypeObject(Box::from(generate_schema(t, all_types)))
        }
        Type::Ref(r) => match r {
            Ref::Type(tpe) => {
                let tpe = all_types
                    .get(tpe)
                    .context(format!("Cannot find type {:?}", tpe))
                    .unwrap();
                match tpe {
                    GlobalType::Object(properties) => ConverterType::Object(
                        properties
                            .iter()
                            .map(|GlobalTypeProperty { name, r#type }| {
                                (name.clone(), generate_schema(r#type, all_types))
                            })
                            .collect(),
                    ),
                    GlobalType::String => ConverterType::String,
                    GlobalType::Boolean => ConverterType::Bool,
                    GlobalType::Number => ConverterType::Double,
                    GlobalType::Integer => ConverterType::Int,
                }
            }
            Ref::Archive => ConverterType::String, // FIXME
            Ref::Asset => ConverterType::String,
            Ref::Any => ConverterType::String,
        },
        Type::Option(t) => ConverterType::Nullable(Box::from(generate_schema(t, all_types))),
    }
}
