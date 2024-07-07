use crate::model::ElementId;
use crate::output::replace_multiple_dashes;
use crate::output::rust::convert_type;
use convert_case::{Case, Casing};
use handlebars::Handlebars;
use serde::Serialize;
use serde_json::json;

static TEMPLATE: &str = include_str!("resource.rs.handlebars");

#[derive(Serialize)]
struct InputProperty {
    name: String,
    arg_name: String,
    type_: String,
    wit_name: String,
}

#[derive(Serialize)]
struct OutputProperty {
    name: String,
    arg_name: String,
    type_: String,
    wit_name: String,
}

#[derive(Serialize)]
struct Interface {
    name: String,
    input_properties: Vec<InputProperty>,
    output_properties: Vec<OutputProperty>,
    struct_name: String,
    function_name: String,
    wit_name: String,
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
                function_name: element_id
                    .name
                    .clone()
                    .from_case(Case::UpperCamel)
                    .to_case(Case::Snake),
                wit_name: create_valid_wit_element_id(element_id),
                input_properties: resource
                    .input_properties
                    .iter()
                    .map(|input_property| InputProperty {
                        name: input_property.name.clone(),
                        arg_name: create_valid_id(&input_property.name),
                        type_: convert_type(&input_property.r#type),
                        wit_name: convert_to_wit_name(&create_valid_wit_id(&input_property.name)),
                    })
                    .collect(),
                output_properties: resource
                    .output_properties
                    .iter()
                    .map(|output_property| OutputProperty {
                        name: output_property.name.clone(),
                        arg_name: create_valid_id(&output_property.name),
                        type_: convert_type(&output_property.r#type),
                        wit_name: convert_to_wit_name(&create_valid_wit_id(&output_property.name)),
                    })
                    .collect(),
            })
            .collect(),
    }
}

fn convert_to_wit_name(s: &str) -> String {
    s.replace('-', "_")
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

fn create_valid_wit_element_id(element_id: &ElementId) -> String {
    let mut vec = element_id.namespace.clone();
    vec.push(element_id.name.clone());
    create_valid_id(&vec.join("-"))
}

fn create_valid_wit_id(s: &str) -> String {
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
    result
}

pub(crate) fn generate_source_code(package: &crate::model::Package) -> String {
    let package = convert_model(package);

    let content = {
        let handlebars = Handlebars::new();
        handlebars
            .render_template(TEMPLATE, &json!({"package": &package}))
            .unwrap()
    };

    content
}
