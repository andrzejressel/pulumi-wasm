use crate::model::{Ref, Type};

pub(crate) mod cargo;
pub(crate) mod source_code_librs;
pub(crate) mod source_code_resource;
pub(crate) mod source_code_types;

fn convert_type(type_or_ref: &Type) -> String {
    match type_or_ref {
        Type::Boolean => "bool".into(),
        Type::Integer => "i32".into(),
        Type::Number => "f64".into(),
        Type::String => "String".into(),
        Type::Array(type_) => format!("Vec<{}>", convert_type(type_)), // "Vec<{}>
        Type::Object(type_) => {
            format!("std::collections::HashMap<String, {}>", convert_type(type_))
        }
        Type::Ref(r) => match r {
            Ref::Type(tpe) => format!("crate::types::{}", tpe.name),
            Ref::Archive => "String".to_string(), //FIXME
            Ref::Asset => "String".to_string(),   //FIXME
            Ref::Any => "String".to_string(),     //FIXME
        },
        Type::Option(type_) => format!("Option<{}>", convert_type(type_)),
    }
}
