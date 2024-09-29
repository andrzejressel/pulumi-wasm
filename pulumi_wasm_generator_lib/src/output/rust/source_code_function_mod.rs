use handlebars::Handlebars;
use serde::Serialize;
use serde_json::json;

static TEMPLATE: &str = include_str!("function_mod.rs.handlebars");

#[derive(Serialize)]
struct Function {
    name: String,
}

#[derive(Serialize)]
struct Package {
    functions: Vec<Function>,
}

fn convert_model(package: &crate::model::Package) -> Package {
    Package {
        functions: package
            .functions
            .keys()
            .map(|(element_id)| Function {
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
