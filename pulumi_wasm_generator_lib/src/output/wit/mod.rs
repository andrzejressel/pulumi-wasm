use crate::model::ElementId;
use crate::output::{get_main_version, get_main_version_stringify, replace_multiple_dashes};
use handlebars::Handlebars;

use serde::Serialize;

static TEMPLATE: &str = include_str!("wit.handlebars");
static DEPENDENCIES: &str = include_str!("dependencies.handlebars");

#[derive(Serialize)]
struct Argument {
    name: String,
    // r#type: String,
}

#[derive(Serialize)]
struct Result {
    name: String,
    // r#type: String,
}

#[derive(Serialize)]
struct Interface {
    name: String,
    arguments: Vec<Argument>,
    results: Vec<Result>,
}

#[derive(Serialize)]
struct Package {
    name: String,
    version: String,
    pulumi_wasm_version: String,
    pulumi_wasm_version_stringify: String,
    interfaces: Vec<Interface>,
}

fn convert_model(package: &crate::model::Package) -> Package {
    Package {
        name: create_valid_id(&package.name),
        version: package.version.clone(),
        pulumi_wasm_version: get_main_version().to_string(),
        pulumi_wasm_version_stringify: get_main_version_stringify().to_string(),
        interfaces: package
            .resources
            .iter()
            .map(|(element_id, resource)| Interface {
                name: create_valid_element_id(element_id),
                arguments: resource
                    .input_properties
                    .iter()
                    .map(|input_property| Argument {
                        name: create_valid_id(&input_property.name),
                    })
                    .collect(),
                results: resource
                    .output_properties
                    .iter()
                    .map(|output_property| Result {
                        name: create_valid_id(&output_property.name),
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
    let result = format!("%{result}");

    result
}

pub(crate) fn generate_wit(package: &crate::model::Package) -> anyhow::Result<String> {
    let mut data = std::collections::BTreeMap::new();
    data.insert("package", convert_model(package));

    let reg = Handlebars::new();
    let output = reg.render_template(TEMPLATE, &data)?;

    Ok(output)
}

pub(crate) fn get_dependencies() -> anyhow::Result<String> {
    let mut data = std::collections::BTreeMap::new();
    data.insert("pulumi_wasm_version", get_main_version());

    let reg = Handlebars::new();
    let output = reg.render_template(DEPENDENCIES, &data)?;

    Ok(output)
}
