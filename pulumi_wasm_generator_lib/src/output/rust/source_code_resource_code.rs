use crate::model::{Resource, Type};
use convert_case::{Case, Casing};
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
    description_lines: Vec<String>,
    optional: bool,
}

#[derive(Serialize)]
struct OutputProperty {
    name: String,
    arg_name: String,
    type_: String,
    description_lines: Vec<String>,
}

#[derive(Serialize)]
struct Interface {
    name: String,
    input_properties: Vec<InputProperty>,
    output_properties: Vec<OutputProperty>,
    struct_name: String,
    function_name: String,
    description_lines: Vec<String>,
    discriminated_unions: Vec<String>,
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
                description_lines: crate::utils::to_lines(
                    resource.description.clone(),
                    package,
                    Some(element_id.clone()),
                ),
                input_properties: resource
                    .input_properties
                    .iter()
                    .map(|input_property| InputProperty {
                        name: input_property.name.clone(),
                        arg_name: input_property.get_rust_argument_name(),
                        optional: matches!(input_property.r#type, Type::Option(_)),
                        type_: input_property.r#type.get_rust_type(),
                        description_lines: crate::utils::to_lines(
                            input_property.description.clone(),
                            package,
                            None,
                        ),
                    })
                    .collect(),
                output_properties: resource
                    .output_properties
                    .iter()
                    .map(|output_property| OutputProperty {
                        name: output_property.name.clone(),
                        arg_name: output_property.get_rust_argument_name(),
                        type_: output_property.r#type.get_rust_type(),
                        description_lines: crate::utils::to_lines(
                            output_property.description.clone(),
                            package,
                            None,
                        ),
                    })
                    .collect(),
                discriminated_unions: get_discriminated_unions(resource),
            })
            .collect(),
    }
}

fn get_discriminated_unions(res: &Resource) -> Vec<String> {
    let mut code: Vec<String> = Vec::new();

    res.input_properties.iter().for_each(|input_property| {
        let tpe = &input_property.r#type;
        let name = format!("{}_{}", "input", input_property.name).to_case(Case::Pascal);
        if let Some(vec) = tpe.get_internal_discriminated_union() {
            code.push(create_discriminated_union(name, &vec));
        }
    });

    code
}

fn create_discriminated_union(name: String, types: &Vec<Type>) -> String {
    let mut code = String::new();

    code.push_str("#[derive(serde::Serialize, serde::Deserialize)]\n");
    code.push_str("#[serde(untagged)]");
    code.push_str(&format!("pub enum {} {{", name));

    types.iter().enumerate().for_each(|(id, tpe)| {
        code.push_str(&format!("    El{}(Box<{}>),", id, tpe.get_rust_type()));
    });

    code.push_str("}\n");

    code
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
