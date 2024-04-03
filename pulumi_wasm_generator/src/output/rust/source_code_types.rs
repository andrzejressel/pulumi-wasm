use crate::model::{ElementId, GlobalType};
use crate::output::replace_multiple_dashes;
use crate::output::rust::convert_type;
use convert_case::{Case, Casing};
use handlebars::Handlebars;
use serde::Serialize;
use serde_json::json;

static TEMPLATE: &str = include_str!("types.rs.handlebars");

#[derive(Serialize)]
struct Property {
    name: String,
    type_: String,
}

#[derive(Serialize)]
struct RefType {
    name: String,
    fields: Vec<Property>,
    struct_name: String,
    function_name: String,
    wit_name: String,
}

#[derive(Serialize)]
struct AliasType {
    name: String,
    type_: String,
}

#[derive(Serialize)]
struct Package {
    name: String,
    types: Vec<RefType>,
    aliases: Vec<AliasType>,
}

fn convert_model(package: &crate::model::Package) -> Package {
    let mut real_types = Vec::new();
    let mut aliases = Vec::new();

    package.types.iter().for_each(|(element_id, resource)| {
        match resource {
            GlobalType::Object(properties) => {
                let ref_type = RefType {
                    name: create_valid_element_id(element_id),
                    struct_name: element_id.name.clone(),
                    function_name: element_id
                        .name
                        .clone()
                        .from_case(Case::UpperCamel)
                        .to_case(Case::Snake),
                    wit_name: create_valid_wit_element_id(element_id),
                    fields: properties
                        .iter()
                        .map(|global_type_property| Property {
                            name: global_type_property.name.clone(),
                            // arg_name: create_valid_id(&global_type_property.name),
                            type_: convert_type(&global_type_property.r#type),
                            // wit_name: convert_to_wit_name(&create_valid_wit_id(&global_type_property.name)),
                        })
                        .collect(),
                };
                real_types.push(ref_type);
            }
            GlobalType::String => aliases.push(AliasType {
                name: element_id.name.to_string(),
                type_: "String".to_string(),
            }),
            GlobalType::Boolean => aliases.push(AliasType {
                name: element_id.name.to_string(),
                type_: "bool".to_string(),
            }),
            GlobalType::Number => aliases.push(AliasType {
                name: element_id.name.to_string(),
                type_: "f64".to_string(),
            }),
            GlobalType::Integer => aliases.push(AliasType {
                name: element_id.name.to_string(),
                type_: "i32".to_string(),
            }),
        }
    });

    Package {
        name: package.name.clone(),
        types: real_types,
        aliases,
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
