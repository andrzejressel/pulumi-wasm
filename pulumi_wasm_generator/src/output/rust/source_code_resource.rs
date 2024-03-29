use crate::model::{ElementId, TypeOrRef, TypeType};
use crate::output::replace_multiple_dashes;
use handlebars::Handlebars;
use serde::Serialize;
use serde_json::json;
use std::path::PathBuf;

static TEMPLATE: &str = include_str!("resource.rs.handlebars");

#[derive(Serialize)]
struct InputProperty {
    name: String,
    arg_name: String,
    type_: String,
}

#[derive(Serialize)]
struct OutputProperty {
    name: String,
    arg_name: String,
}

#[derive(Serialize)]
struct Interface {
    name: String,
    r#type: String,
    input_properties: Vec<InputProperty>,
    output_properties: Vec<OutputProperty>,
    struct_name: String,
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
                struct_name: element_id.name.clone(),
                r#type: element_id.raw.clone(),
                input_properties: resource
                    .input_properties
                    .iter()
                    .map(|input_property| InputProperty {
                        name: input_property.name.clone(),
                        arg_name: create_valid_id(&input_property.name),
                        type_: convert_typeofref(&input_property.r#type),
                    })
                    .collect(),
                output_properties: resource
                    .output_properties
                    .iter()
                    .map(|output_property| OutputProperty {
                        name: output_property.name.clone(),
                        arg_name: create_valid_id(&output_property.name),
                    })
                    .collect(),
            })
            .collect(),
    }
}

fn convert_typeofref(type_or_ref: &TypeOrRef) -> String {
    match type_or_ref {
        TypeOrRef::Type(tt) => match tt {
            TypeType::Boolean => "boolean".into(),
            TypeType::Integer => "int".into(),
            TypeType::Number => "double".into(),
            TypeType::String => "String".into(),
            TypeType::Array => "Vec".into(),
            TypeType::Object => "Object".into(),
        },
        TypeOrRef::Ref(r) => format!("Ref<{}>", r),
    }
}

fn create_valid_element_id(element_id: &ElementId) -> String {
    let mut vec = element_id.namespace.clone();
    vec.push(element_id.name.clone());
    create_valid_id(&vec.join("-"))
}

fn create_valid_id(s: &String) -> String {
    let result = s
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

pub(crate) fn generate_source_code(package: &crate::model::Package) -> Vec<(PathBuf, String)> {
    let package = convert_model(package);

    let element = package
        .interfaces
        .iter()
        .map(|interface| {
            let path = PathBuf::from(format!("{}.rs", interface.name));
            let handlebars = Handlebars::new();
            let content = handlebars
                .render_template(TEMPLATE, &json!({"interface": &interface}))
                .unwrap();
            (path, content)
        })
        .collect();

    element
    // let handlebars = Handlebars::new();
    // let el = ("".into(), handlebars.render_template(TEMPLATE, &json!({"package": &convert_model(package)})).unwrap());
    // vec![el]
}
