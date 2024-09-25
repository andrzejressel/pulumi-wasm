use handlebars::Handlebars;
use serde::Serialize;
use serde_json::json;

static TEMPLATE: &str = include_str!("resource_mod.rs.handlebars");

#[derive(Serialize)]
struct Interface {
    name: String,
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
            })
            .collect(),
    }
}

pub(crate) fn generate_source_code(package: &crate::model::Package) -> String {
    let package = convert_model(package);

    let content = {
        let handlebars = Handlebars::new();
        handlebars
            .render_template(TEMPLATE, &json!({"package": &package}))
            .unwrap()
    };

    content
}
