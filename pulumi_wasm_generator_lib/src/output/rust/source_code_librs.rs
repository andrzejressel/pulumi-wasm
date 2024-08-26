use crate::output::get_main_version;
use handlebars::Handlebars;
use serde::Serialize;
use serde_json::json;

static TEMPLATE: &str = include_str!("lib.rs.handlebars");

#[derive(Serialize)]
struct Interface {}

#[derive(Serialize)]
struct Package {
    name: String,
    contains_resources: bool,
    contains_types: bool,
    pulumi_wasm_version: String,
}

fn convert_model(package: &crate::model::Package) -> Package {
    Package {
        name: package.name.clone(),
        pulumi_wasm_version: get_main_version().to_string(),
        contains_types: !package.types.is_empty(),
        contains_resources: !package.resources.is_empty(),
    }
}

pub(crate) fn generate_source_code(package: &crate::model::Package) -> String {
    let handlebars = Handlebars::new();
    handlebars
        .render_template(TEMPLATE, &json!({"package": &convert_model(package)}))
        .unwrap()
}
