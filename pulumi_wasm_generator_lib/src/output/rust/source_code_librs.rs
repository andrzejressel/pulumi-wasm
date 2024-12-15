use std::collections::BTreeSet;
use crate::output::get_main_version;
use handlebars::Handlebars;
use serde::Serialize;
use serde_json::json;
use crate::model::GlobalType;
use itertools::Itertools;

static TEMPLATE: &str = include_str!("lib.rs.handlebars");

#[derive(Serialize)]
struct Interface {}

#[derive(Serialize)]
struct Package {
    name: String,
    contains_elements: bool,
    contains_types: bool,
    pulumi_wasm_version: String,
    contains_resources: bool,
    contains_functions: bool,
    const_strings: Vec<String>,
}

fn convert_model(package: &crate::model::Package) -> Package {
    Package {
        name: package.name.clone(),
        pulumi_wasm_version: get_main_version().to_string(),
        contains_types: !package.types.is_empty(),
        contains_elements: !package.resources.is_empty() || !package.functions.is_empty(),
        contains_resources: !package.resources.is_empty(),
        contains_functions: !package.functions.is_empty(),
        const_strings: find_consts(package),
    }
}

fn find_consts(package: &crate::model::Package) -> Vec<String> {
    let mut consts = BTreeSet::new();
    for (_, resource) in &package.resources {
        for (input) in &resource.input_properties {
            consts.extend(input.r#type.get_consts().clone());
        }
        for (output) in &resource.output_properties {
            consts.extend(output.r#type.get_consts().clone());
        }
    }
    for (_, function) in &package.functions {
        for (input) in &function.input_properties {
            consts.extend(input.r#type.get_consts().clone());
        }
        for (output) in &function.output_properties {
            consts.extend(output.r#type.get_consts().clone());
        }
    }
    for (_, type_) in &package.types {
        if let GlobalType::Object(_, obj) = type_ {
            for gtp in obj {
                consts.extend(gtp.r#type.get_consts().clone());
            }
        }
    }
    consts.into_iter().sorted().collect()
}

pub(crate) fn generate_source_code(package: &crate::model::Package) -> String {
    let handlebars = Handlebars::new();
    handlebars
        .render_template(TEMPLATE, &json!({"package": &convert_model(package)}))
        .unwrap()
}
