use handlebars::Handlebars;

use serde::Serialize;
use serde_json::json;

static TEMPLATE: &str = include_str!("lib.rs.handlebars");

#[derive(Serialize)]
struct Package {
    name: String,
    name_escaped: String,
    contains_resources: bool,
    contains_functions: bool,
    contains_resources_or_functions: bool,
}

fn convert_model(package: &crate::model::Package) -> Package {
    Package {
        name: package.name.clone(),
        name_escaped: package.name.clone().replace('-', "_"),
        contains_resources: !package.resources.is_empty() || !package.functions.is_empty(),
        contains_functions: !package.functions.is_empty(),
        contains_resources_or_functions: !package.resources.is_empty()
            || !package.functions.is_empty(),
    }
}

pub(crate) fn generate_source_code(package: &crate::model::Package) -> String {
    let handlebars = Handlebars::new();
    handlebars
        .render_template(TEMPLATE, &json!({"package": &convert_model(package)}))
        .unwrap()
}
