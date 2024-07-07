use handlebars::Handlebars;
use serde::Serialize;
use serde_json::json;

static TEMPLATE: &str = include_str!("Cargo.toml.handlebars");

#[derive(Serialize)]
struct Package {
    name: String,
    version: String,
}

fn convert_model(package: &crate::model::Package) -> Package {
    Package {
        name: package.name.clone(),
        version: package.version.clone(),
    }
}

pub(crate) fn generate_cargo(package: &crate::model::Package) -> String {
    let package = convert_model(package);
    let handlebars = Handlebars::new();
    handlebars
        .render_template(TEMPLATE, &json!({"package": &package}))
        .unwrap()
}
