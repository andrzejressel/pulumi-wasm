use crate::model::{GlobalType, Type};
use convert_case::{Case, Casing};
use handlebars::Handlebars;
use serde::Serialize;
use serde_json::json;
use std::collections::HashMap;
use std::path::PathBuf;

static TEMPLATE: &str = include_str!("types_code.rs.handlebars");

#[derive(Serialize)]
struct Property {
    name: String,
    original_name: String,
    type_: String,
    description_lines: Vec<String>,
    optional: bool,
}

#[derive(Serialize)]
struct RefType {
    // name: String,
    fields: Vec<Property>,
    struct_name: String,
    file_name: String,
}

#[derive(Serialize)]
struct AliasType {
    name: String,
    type_: String,
}

#[derive(Serialize)]
struct Package {
    name: String,
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
            GlobalType::Object(properties) => {
                let ref_type = RefType {
                    struct_name: element_id.get_rust_struct_name(),
                    file_name: element_id.get_rust_struct_name().to_case(Case::Snake),
                    fields: properties
                        .iter()
                        .map(|global_type_property| Property {
                            name: global_type_property
                                .name
                                .clone()
                                .from_case(Case::Camel)
                                .to_case(Case::Snake),
                            original_name: global_type_property.name.clone(),
                            type_: global_type_property.r#type.get_rust_type(),
                            optional: matches!(global_type_property.r#type, Type::Option(_)),
                            description_lines: crate::utils::to_lines(
                                global_type_property.description.clone(),
                                package,
                            ),
                        })
                        .collect(),
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
        name: package.name.clone(),
        types: real_types,
        aliases,
    }
}

pub(crate) fn generate_source_code(package: &crate::model::Package) -> HashMap<PathBuf, String> {
    let handlebars = Handlebars::new();
    let package = convert_model(package);

    package
        .types
        .iter()
        .map(|type_| {
            let rendered_file = handlebars
                .render_template(TEMPLATE, &json!({"type": type_}))
                .unwrap();
            (
                PathBuf::from(format!("{}.rs", type_.file_name)),
                rendered_file,
            )
        })
        .collect()
}
