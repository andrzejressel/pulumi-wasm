use handlebars::Handlebars;
use serde::Serialize;
use serde_json::json;
use std::collections::HashMap;
use std::path::PathBuf;

static TEMPLATE: &str = include_str!("resource_code.rs.handlebars");

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
    type_: String,
}

#[derive(Serialize)]
struct Interface {
    name: String,
    input_properties: Vec<InputProperty>,
    output_properties: Vec<OutputProperty>,
    struct_name: String,
    function_name: String,
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
                name: element_id.get_rust_namespace_name(),
                struct_name: element_id.name.clone(),
                function_name: element_id.get_rust_function_name(),
                input_properties: resource
                    .input_properties
                    .iter()
                    .map(|input_property| InputProperty {
                        name: input_property.name.clone(),
                        arg_name: input_property.get_rust_argument_name(),
                        type_: input_property.r#type.get_rust_type(),
                    })
                    .collect(),
                output_properties: resource
                    .output_properties
                    .iter()
                    .map(|output_property| OutputProperty {
                        name: output_property.name.clone(),
                        arg_name: output_property.get_rust_argument_name(),
                        type_: output_property.r#type.get_rust_type(),
                    })
                    .collect(),
            })
            .collect(),
    }
}

pub(crate) fn generate_source_code(package: &crate::model::Package) -> HashMap<PathBuf, String> {
    let handlebars = Handlebars::new();
    let package = convert_model(package);

    package
        .interfaces
        .iter()
        .map(|interface| {
            let rendered_file = handlebars
                .render_template(
                    TEMPLATE,
                    &json!({"package_name": package.name, "interface": interface}),
                )
                .unwrap();
            (
                PathBuf::from(format!("{}.rs", interface.name)),
                rendered_file,
            )
        })
        .collect()
}