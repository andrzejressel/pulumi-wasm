use crate::model::{GlobalType, Type};
use convert_case::{Case, Casing};
use handlebars::Handlebars;
use serde::Serialize;
use serde_json::json;
use std::collections::{BTreeSet, HashMap};
use std::path::PathBuf;

static TEMPLATE: &str = include_str!("types_code.rs.handlebars");
static ENUM_TEMPLATE: &str = include_str!("types_code_enum.rs.handlebars");

#[derive(Serialize)]
struct Property {
    name: String,
    original_name: String,
    type_: String,
    description_lines: Vec<String>,
    default: bool,
    skip: bool,
    private: bool,
}

#[derive(Serialize)]
struct RefType {
    // name: String,
    fields: Vec<Property>,
    description_lines: Vec<String>,
    struct_name: String,
    file_name: String,
    const_strings: BTreeSet<String>,
}

#[derive(Serialize)]
struct Enum {
    struct_name: String,
    file_name: String,
    description_lines: Vec<String>,
    values: Vec<EnumValue>,
}

#[derive(Serialize)]
struct EnumValue {
    name: String,
    description_lines: Vec<String>,
    value: Option<String>,
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
    enums: Vec<Enum>,
}

fn convert_model(package: &crate::model::Package) -> Package {
    let mut real_types = Vec::new();
    let mut enums = Vec::new();

    package
        .types
        .iter()
        .for_each(|(element_id, resource)| match resource {
            GlobalType::Object(description, properties) => {
                let ref_type = RefType {
                    struct_name: element_id.get_rust_struct_name(),
                    file_name: element_id.get_rust_struct_name().to_case(Case::Snake),
                    description_lines: crate::utils::to_lines(description.clone(), package, None),
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
                            default: matches!(global_type_property.r#type, Type::Option(_)),
                            skip: matches!(global_type_property.r#type, Type::ConstString(_)),
                            private: matches!(global_type_property.r#type, Type::ConstString(_)),
                            description_lines: crate::utils::to_lines(
                                global_type_property.description.clone(),
                                package,
                                None,
                            ),
                        })
                        .collect(),
                    const_strings: properties
                        .iter()
                        .flat_map(|global_type_property| global_type_property.r#type.get_consts())
                        .collect(),
                };
                real_types.push(ref_type);
            }
            GlobalType::StringEnum(description, enum_values) => {
                let enum_type = Enum {
                    struct_name: element_id.get_rust_struct_name(),
                    file_name: element_id.get_rust_struct_name().to_case(Case::Snake),
                    description_lines: crate::utils::to_lines(description.clone(), package, None),
                    values: enum_values
                        .iter()
                        .map(|enum_value| EnumValue {
                            name: enum_value.name.clone(),
                            value: enum_value.value.clone(),
                            description_lines: crate::utils::to_lines(
                                enum_value.description.clone(),
                                package,
                                None,
                            ),
                        })
                        .collect(),
                };
                enums.push(enum_type);
            }
            GlobalType::String => {}
            GlobalType::Boolean => {}
            GlobalType::Number => {}
            GlobalType::Integer => {}
        });

    Package {
        name: package.name.clone(),
        types: real_types,
        enums,
    }
}

pub(crate) fn generate_source_code(package: &crate::model::Package) -> HashMap<PathBuf, String> {
    let handlebars = Handlebars::new();
    let package = convert_model(package);

    let types: HashMap<_, _> = package
        .types
        .iter()
        .map(|type_| {
            let rendered_file = handlebars
                .render_template(TEMPLATE, &json!({"type": type_}))
                .unwrap()
                .trim_start()
                .to_string(); //FIXME
            (
                PathBuf::from(format!("{}.rs", type_.file_name)),
                rendered_file,
            )
        })
        .collect();

    let enums: HashMap<_, _> = package
        .enums
        .iter()
        .map(|enum_| {
            let rendered_file = handlebars
                .render_template(ENUM_TEMPLATE, &json!({"enum": enum_}))
                .unwrap()
                .trim_start()
                .to_string(); //FIXME
            (
                PathBuf::from(format!("{}.rs", enum_.file_name)),
                rendered_file,
            )
        })
        .collect();

    let mut result = HashMap::new();
    result.extend(types);
    result.extend(enums);

    result

    // let enums =
}
