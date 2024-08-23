use handlebars::Handlebars;
use std::collections::HashMap;
use std::path::PathBuf;

use serde::Serialize;
use serde_json::json;

static TEMPLATE: &str = include_str!("resource.rs.handlebars");

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
    package_name: String,
    name: String,
    r#type: String,
    input_properties: Vec<InputProperty>,
    output_properties: Vec<OutputProperty>,
}

fn convert_model(package: &crate::model::Package) -> Vec<Interface> {
    package
        .resources
        .iter()
        .map(|(element_id, resource)| Interface {
            package_name: package.name.clone().replace('-', "_"),
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
        .collect()
}

pub(crate) fn generate_source_code(package: &crate::model::Package) -> HashMap<PathBuf, String> {
    let handlebars = Handlebars::new();
    let interfaces = convert_model(package);

    interfaces
        .iter()
        .map(|interface| {
            let rendered_file = handlebars
                .render_template(TEMPLATE, &json!({"interface": interface}))
                .unwrap();
            (
                PathBuf::from(format!("{}.rs", interface.name)),
                rendered_file,
            )
        })
        .collect()
}
