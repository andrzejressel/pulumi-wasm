use crate::output::get_main_version;
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
struct Interface {}

#[derive(Serialize)]
struct Package {
    name: String,
    interfaces: Vec<Interface>,
    pulumi_wasm_version: String,
}

fn convert_model(package: &crate::model::Package) -> Package {
    Package {
        name: package.name.clone(),
        pulumi_wasm_version: get_main_version().to_string(),
        interfaces: package
            .resources
            .iter()
            .map(|(element_id, resource)| Interface {})
            .collect(),
    }
}

pub(crate) fn generate_source_code(package: &crate::model::Package) -> String {
    let handlebars = Handlebars::new();
    handlebars
        .render_template(TEMPLATE, &json!({"package": &convert_model(package)}))
        .unwrap()
}
