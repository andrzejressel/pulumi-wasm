use handlebars::Handlebars;

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
    name_escaped: String,
    contains_resources: bool,
}

fn convert_model(package: &crate::model::Package) -> Package {
    Package {
        name: package.name.clone(),
        name_escaped: package.name.clone().replace('-', "_"),
        contains_resources: !package.resources.is_empty() || !package.functions.is_empty(),
        interfaces: package
            .resources
            .iter()
            .map(|(element_id, resource)| Interface {
                name: element_id.get_rust_namespace_name(),
                r#type: element_id.raw.clone(),
                input_properties: resource
                    .input_properties
                    .iter()
                    .map(|input_property| InputProperty {
                        name: input_property.name.clone(),
                        arg_name: input_property.get_rust_argument_name(),
                    })
                    .collect(),
                output_properties: resource
                    .output_properties
                    .iter()
                    .map(|output_property| OutputProperty {
                        name: output_property.name.clone(),
                        arg_name: output_property.get_rust_argument_name(),
                    })
                    .collect(),
            })
            .collect(),
    }
}

pub(crate) fn generate_source_code(package: &crate::model::Package) -> String {
    let handlebars = Handlebars::new();
    handlebars
        .render_template(TEMPLATE, &json!({"package": &convert_model(package)}))
        .unwrap()
}
