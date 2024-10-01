use crate::output::get_main_version;
use handlebars::Handlebars;

use serde::Serialize;

static TEMPLATE: &str = include_str!("wit.handlebars");
static DEPENDENCIES: &str = include_str!("dependencies.handlebars");

#[derive(Serialize)]
struct Argument {
    name: String,
}

#[derive(Serialize)]
struct Result {
    name: String,
}

#[derive(Serialize)]
struct Resources {
    name: String,
    arguments: Vec<Argument>,
    results: Vec<Result>,
}

#[derive(Serialize)]
struct Function {
    name: String,
    arguments: Vec<Argument>,
    results: Vec<Result>,
}

#[derive(Serialize)]
struct Package {
    name: String,
    version: String,
    pulumi_wasm_version: String,
    resources: Vec<Resources>,
    functions: Vec<Function>,
}

fn convert_model(package: &crate::model::Package) -> Package {
    let resources: Vec<_> = package
        .resources
        .iter()
        .map(|(element_id, resource)| Resources {
            name: element_id.get_wit_interface_name(),
            arguments: resource
                .input_properties
                .iter()
                .map(|input_property| Argument {
                    name: input_property.get_wit_argument_name(),
                })
                .collect(),
            results: resource
                .output_properties
                .iter()
                .map(|output_property| Result {
                    name: output_property.get_wit_argument_name(),
                })
                .collect(),
        })
        .collect();

    let functions: Vec<_> = package
        .functions
        .iter()
        .map(|(element_id, resource)| Function {
            name: element_id.get_wit_interface_name(),
            arguments: resource
                .input_properties
                .iter()
                .map(|input_property| Argument {
                    name: input_property.get_wit_argument_name(),
                })
                .collect(),
            results: resource
                .output_properties
                .iter()
                .map(|output_property| Result {
                    name: output_property.get_wit_argument_name(),
                })
                .collect(),
        })
        .collect();

    Package {
        name: package.get_wit_name(),
        version: package.version.clone(),
        pulumi_wasm_version: get_main_version().to_string(),
        resources,
        functions,
    }
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
