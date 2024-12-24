use crate::model::{ElementId, GlobalType, Type};
use convert_case::{Case, Casing};
use handlebars::Handlebars;
use serde::Serialize;
use serde_json::json;
use std::collections::BTreeSet;

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

enum GenerateResource {
    RealType(RefType),
    StringEnum(StringEnum),
    NumberEnum(NumberEnum),
    IntegerEnum(IntegerEnum),
}

fn convert_resource(package: &crate::model::Package, element_id: &ElementId) -> GenerateResource {
    let resource = package.types.get(element_id).unwrap();
    let depth = element_id.namespace.len() + 1;
    match resource {
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
                        type_: global_type_property.r#type.get_rust_type(depth),
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
            GenerateResource::RealType(ref_type)
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
            GenerateResource::StringEnum(enum_type)
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
            GenerateResource::NumberEnum(enum_type)
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
            GenerateResource::IntegerEnum(enum_type)
        }
    }
}

pub(crate) fn generate_single_type_source_file(
    package: &crate::model::Package,
    element_id: &ElementId,
) -> String {
    let handlebars = Handlebars::new();

    match convert_resource(package, element_id) {
        GenerateResource::RealType(type_) => handlebars
            .render_template(TEMPLATE, &json!({"type": type_}))
            .unwrap()
            .trim_start()
            .to_string(),
        GenerateResource::StringEnum(enum_) => handlebars
            .render_template(STRING_ENUM_TEMPLATE, &json!({"enum": enum_}))
            .unwrap()
            .trim_start()
            .to_string(),
        GenerateResource::NumberEnum(enum_) => handlebars
            .render_template(NUMBER_ENUM_TEMPLATE, &json!({"enum": enum_}))
            .unwrap()
            .trim_start()
            .to_string(),
        GenerateResource::IntegerEnum(enum_) => handlebars
            .render_template(INTEGER_ENUM_TEMPLATE, &json!({"enum": enum_}))
            .unwrap()
            .trim_start()
            .to_string(),
    }
}

pub(crate) fn generate_docs(
    package: &crate::model::Package,
    element_id: &ElementId,
) -> Vec<String> {
    let resource = convert_resource(package, element_id);
    match resource {
        GenerateResource::RealType(a) => a.description_lines,
        GenerateResource::StringEnum(a) => a.description_lines,
        GenerateResource::NumberEnum(a) => a.description_lines,
        GenerateResource::IntegerEnum(a) => a.description_lines,
    }
}
