use handlebars::Handlebars;
use regex::Regex;
use serde::Serialize;
use serde_json::json;

static TEMPLATE: &'static str = include_str!("lib.rs.handlebars");

#[derive(Serialize)]
struct InputProperty {
    name: String,
    arg_name: String,
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
}

#[derive(Serialize)]
struct Package {
    name: String,
    interfaces: Vec<Interface>,
}

fn convert_model(package: &crate::model::Package) -> Package {
    Package {
        name: package.name.clone(),
        interfaces: package.resources.iter().map(|resource| {
            Interface {
                name: create_valid_id(&resource.name),
                r#type: resource.name.clone(),
                input_properties: resource.input_properties.iter().map(|input_property| {
                    InputProperty {
                        name: input_property.name.clone(),
                        arg_name: create_valid_id(&input_property.name)
                    }
                }).collect(),
                output_properties: resource.output_properties.iter().map(|output_property| {
                    OutputProperty {
                        name: output_property.name.clone(),
                        arg_name: create_valid_id(&output_property.name),
                    }
                }).collect(),
            }
        }).collect()
    }
}

fn create_valid_id(s: &String) -> String {
    let result = s.chars()
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
    let result = result.replace("-", "_");

    result
}

fn replace_multiple_dashes(s: &String) -> String {
    let re = Regex::new("-+").unwrap();
    let result = re.replace_all(s, "-");
    result.to_string()
}

pub(crate) fn generate_source_code(package: &crate::model::Package) -> String {
    let handlebars = Handlebars::new();
    handlebars.render_template(TEMPLATE, &json!({"package": &convert_model(package)})).unwrap()
}