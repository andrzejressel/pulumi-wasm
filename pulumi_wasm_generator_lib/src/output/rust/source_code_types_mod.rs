use crate::model::GlobalType;
use convert_case::{Case, Casing};
use handlebars::Handlebars;
use serde::Serialize;
use serde_json::json;

static TEMPLATE: &str = include_str!("types_mod.rs.handlebars");

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
}

fn convert_model(package: &crate::model::Package) -> Package {
    let mut real_types = Vec::new();

    package
        .types
        .iter()
        .for_each(|(element_id, resource)| match resource {
            GlobalType::Object(_, _) => {
                let ref_type = RefType {
                    file_name: element_id.get_rust_struct_name().to_case(Case::Snake),
                };
                real_types.push(ref_type);
            }
            GlobalType::StringEnum(_, _)
            | GlobalType::IntegerEnum(_, _)
            | GlobalType::NumberEnum(_, _) => {
                let ref_type = RefType {
                    file_name: element_id.get_rust_struct_name().to_case(Case::Snake),
                };
                real_types.push(ref_type);
            }
        });

    Package { types: real_types }
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
