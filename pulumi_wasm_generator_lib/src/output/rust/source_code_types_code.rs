use crate::model::{GlobalType, Type};
use convert_case::{Case, Casing};
use handlebars::Handlebars;
use serde::Serialize;
use serde_json::json;
use std::collections::{BTreeSet, HashMap};
use std::path::PathBuf;

static TEMPLATE: &str = include_str!("types_code.rs.handlebars");
static STRING_ENUM_TEMPLATE: &str = include_str!("types_code_string_enum.rs.handlebars");
static NUMBER_ENUM_TEMPLATE: &str = include_str!("types_code_number_enum.rs.handlebars");
static INTEGER_ENUM_TEMPLATE: &str = include_str!("types_code_integer_enum.rs.handlebars");

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
struct StringEnum {
    struct_name: String,
    file_name: String,
    description_lines: Vec<String>,
    values: Vec<StringEnumValue>,
}

#[derive(Serialize)]
struct StringEnumValue {
    name: String,
    description_lines: Vec<String>,
    value: Option<String>,
}

#[derive(Serialize)]
struct IntegerEnum {
    struct_name: String,
    file_name: String,
    description_lines: Vec<String>,
    values: Vec<IntegerEnumValue>,
}

#[derive(Serialize)]
struct IntegerEnumValue {
    name: String,
    description_lines: Vec<String>,
    value: i64,
}

#[derive(Serialize)]
struct NumberEnum {
    struct_name: String,
    file_name: String,
    description_lines: Vec<String>,
    values: Vec<NumberEnumValue>,
}

#[derive(Serialize)]
struct NumberEnumValue {
    name: String,
    description_lines: Vec<String>,
    value: f64,
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
    string_enums: Vec<StringEnum>,
    number_enums: Vec<NumberEnum>,
    integer_enums: Vec<IntegerEnum>,
}

fn convert_model(package: &crate::model::Package) -> Package {
    let mut real_types = Vec::new();
    let mut string_enums = Vec::new();
    let mut number_enums = Vec::new();
    let mut integer_enums = Vec::new();

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
                let enum_type = StringEnum {
                    struct_name: element_id.get_rust_struct_name(),
                    file_name: element_id.get_rust_struct_name().to_case(Case::Snake),
                    description_lines: crate::utils::to_lines(description.clone(), package, None),
                    values: enum_values
                        .iter()
                        .map(|enum_value| StringEnumValue {
                            name: enum_value.name.clone(),
                            value: enum_value.value.clone().map(|s| format!("\"{}\"", s)),
                            description_lines: crate::utils::to_lines(
                                enum_value.description.clone(),
                                package,
                                None,
                            ),
                        })
                        .collect(),
                };
                string_enums.push(enum_type);
            }
            GlobalType::NumberEnum(description, enum_values) => {
                let enum_type = NumberEnum {
                    struct_name: element_id.get_rust_struct_name(),
                    file_name: element_id.get_rust_struct_name().to_case(Case::Snake),
                    description_lines: crate::utils::to_lines(description.clone(), package, None),
                    values: enum_values
                        .iter()
                        .map(|enum_value| NumberEnumValue {
                            name: enum_value.name.clone(),
                            value: enum_value.value,
                            description_lines: crate::utils::to_lines(
                                enum_value.description.clone(),
                                package,
                                None,
                            ),
                        })
                        .collect(),
                };
                number_enums.push(enum_type);
            }
            GlobalType::IntegerEnum(description, enum_values) => {
                let enum_type = IntegerEnum {
                    struct_name: element_id.get_rust_struct_name(),
                    file_name: element_id.get_rust_struct_name().to_case(Case::Snake),
                    description_lines: crate::utils::to_lines(description.clone(), package, None),
                    values: enum_values
                        .iter()
                        .map(|enum_value| IntegerEnumValue {
                            name: enum_value.name.clone(),
                            value: enum_value.value,
                            description_lines: crate::utils::to_lines(
                                enum_value.description.clone(),
                                package,
                                None,
                            ),
                        })
                        .collect(),
                };
                integer_enums.push(enum_type);
            }
        });

    Package {
        name: package.name.clone(),
        types: real_types,
        string_enums,
        number_enums,
        integer_enums,
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

    let string_enums: HashMap<_, _> = package
        .string_enums
        .iter()
        .map(|enum_| {
            let rendered_file = handlebars
                .render_template(STRING_ENUM_TEMPLATE, &json!({"enum": enum_}))
                .unwrap()
                .trim_start()
                .to_string(); //FIXME
            (
                PathBuf::from(format!("{}.rs", enum_.file_name)),
                rendered_file,
            )
        })
        .collect();

    let number_enums: HashMap<_, _> = package
        .number_enums
        .iter()
        .map(|enum_| {
            let rendered_file = handlebars
                .render_template(NUMBER_ENUM_TEMPLATE, &json!({"enum": enum_}))
                .unwrap()
                .trim_start()
                .to_string(); //FIXME
            (
                PathBuf::from(format!("{}.rs", enum_.file_name)),
                rendered_file,
            )
        })
        .collect();

    let integer_enums: HashMap<_, _> = package
        .integer_enums
        .iter()
        .map(|enum_| {
            let rendered_file = handlebars
                .render_template(INTEGER_ENUM_TEMPLATE, &json!({"enum": enum_}))
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
    result.extend(string_enums);
    result.extend(number_enums);
    result.extend(integer_enums);

    result

    // let enums =
}
