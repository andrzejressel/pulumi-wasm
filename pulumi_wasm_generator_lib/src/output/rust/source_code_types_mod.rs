use crate::model::GlobalType;
use convert_case::{Case, Casing};
use handlebars::Handlebars;
use serde::Serialize;
use serde_json::json;

static TEMPLATE: &str = include_str!("types_mod.rs.handlebars");

#[derive(Serialize)]
struct Property {
    name: String,
    original_name: String,
    type_: String,
}

#[derive(Serialize)]
struct RefType {
    file_name: String,
}

#[derive(Serialize)]
struct AliasType {
    name: String,
    type_: String,
}

#[derive(Serialize)]
struct Package {
    types: Vec<RefType>,
    aliases: Vec<AliasType>,
}

fn convert_model(package: &crate::model::Package) -> Package {
    let mut real_types = Vec::new();
    let mut aliases = Vec::new();

    package
        .types
        .iter()
        .for_each(|(element_id, resource)| match resource {
            GlobalType::Object(_properties) => {
                let ref_type = RefType {
                    file_name: element_id.get_rust_struct_name().to_case(Case::Snake),
                };
                real_types.push(ref_type);
            }
            GlobalType::String => aliases.push(AliasType {
                name: element_id.name.to_string(),
                type_: "String".to_string(),
            }),
            GlobalType::Boolean => aliases.push(AliasType {
                name: element_id.name.to_string(),
                type_: "bool".to_string(),
            }),
            GlobalType::Number => aliases.push(AliasType {
                name: element_id.name.to_string(),
                type_: "f64".to_string(),
            }),
            GlobalType::Integer => aliases.push(AliasType {
                name: element_id.name.to_string(),
                type_: "i32".to_string(),
            }),
        });

    Package {
        types: real_types,
        aliases,
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
