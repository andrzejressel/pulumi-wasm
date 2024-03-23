use handlebars::Handlebars;
use serde::Serialize;

static TEMPLATE: &'static str = include_str!("wit.handlebars");

#[derive(Serialize)]
struct Argument {
    name: String,
    r#type: String,
}

#[derive(Serialize)]
struct Result {
    name: String,
    r#type: String,
}

#[derive(Serialize)]
struct Interface {
    name: String,
    arguments: Vec<Argument>,
    results: Vec<Result>,
}

#[derive(Serialize)]
struct Package {
    name: String,
    version: String,
    interfaces: Vec<Interface>,
}

fn convert_model(package: &crate::model::Package) -> Package {
    todo!()
}

pub(crate) fn generate_wit(package: &crate::model::Package) -> anyhow::Result<String> {
    let mut data = std::collections::HashMap::new();
    data.insert("package", convert_model(package));

    let reg = Handlebars::new();
    let output = reg.render_template(TEMPLATE, &data)?;

    Ok(output)
}