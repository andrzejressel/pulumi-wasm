use handlebars::Handlebars;
use serde::Serialize;
use regex::Regex;

static TEMPLATE: &'static str = include_str!("wit.handlebars");
static DEPENDENCIES: &'static str = include_str!("dependencies.wit");


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
    interfaces: Vec<Interface>,
}

fn convert_model(package: &crate::model::Package) -> Package {
    Package {
        name: create_valid_id(&package.name),
        version: package.version.clone(),
        interfaces: package.resources.iter().map(|resource| {
            Interface {
                name: create_valid_id(&resource.name),
                arguments: resource.input_properties.iter().map(|input_property| {
                    Argument {
                        name: create_valid_id(&input_property.name),
                    }
                }).collect(),
                results: vec![],
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

    replace_multiple_dashes(&result)
}

fn replace_multiple_dashes(s: &String) -> String {
    let re = Regex::new("-+").unwrap();
    let result = re.replace_all(s, "-");
    result.to_string()
}

pub(crate) fn generate_wit(package: &crate::model::Package) -> anyhow::Result<String> {
    let mut data = std::collections::HashMap::new();
    data.insert("package", convert_model(package));

    let reg = Handlebars::new();
    let output = reg.render_template(TEMPLATE, &data)?;

    Ok(output)
}

pub (crate) fn get_dependencies() -> &'static str {
    return DEPENDENCIES
}