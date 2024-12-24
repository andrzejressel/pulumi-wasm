use crate::output::get_main_version;
use handlebars::Handlebars;
use serde::Serialize;
use serde_json::json;

static TEMPLATE: &str = include_str!("Cargo.toml.handlebars");

#[derive(Serialize)]
struct Package {
    name: String,
    version: String,
    pulumi_wasm_version: String,
}

fn convert_model(package: &crate::model::Package) -> Package {
    Package {
        name: package.name.clone(),
        version: package.version.clone(),
        pulumi_wasm_version: get_main_version().to_string(),
    }
}

pub(crate) fn generate_cargo(package: &crate::model::Package) -> String {
    let package = convert_model(package);
    let handlebars = Handlebars::new();
    handlebars
        .render_template(TEMPLATE, &json!({"package": &package}))
        .unwrap()
}
