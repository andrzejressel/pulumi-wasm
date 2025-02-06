use crate::code_generation::yaml::model::Expression;
use crate::code_generation::yaml::model::Resource;
use crate::code_generation::yaml::model::{Example, FnInvoke, Variable};
use crate::model::ElementId;
use crate::utils::{escape_rust_name, reformat_code};
use anyhow::Context;
use anyhow::Result;
use convert_case::Case;
use convert_case::Casing;
use quote::ToTokens;
use std::collections::BTreeMap;
use syn::LitStr;

pub fn generate_code(example: Example) -> Result<String> {
    let mut result = r"
use pulumi_wasm_rust::Output;
use pulumi_wasm_rust::{add_export, pulumi_main};
#[pulumi_main]
fn test_main() -> Result<(), Error> {
    "
    .trim()
    .to_string();
    result.push('\n');

    for (name, variable) in example.variables {
        result.push_str(generate_variable(name, variable).as_str());
        result.push('\n');
    }

    for (name, resource) in example.resources {
        result.push_str(generate_resource(name, resource).as_str());
        result.push('\n');
    }

    result.push('}');

    let formatted = reformat_code(result.as_str())
        .context(format!("Failed to parse generated Rust code:\n{}", result))?;
    Ok(formatted)
}

fn generate_variable(name: String, variable: Variable) -> String {
    match variable {
        Variable::FnInvokeVariable(fn_invoke) => generate_fn_invoke(name, fn_invoke),
    }
}

fn generate_fn_invoke(name: String, fn_invoke: FnInvoke) -> String {
    let mut str = String::new();
    str.push_str(&format!(
        r#"
    let {} = {}::invoke(
        {}Args::builder()
        "#,
        name,
        fn_invoke.function.get_rust_function_name(),
        fn_invoke.function.get_rust_struct_name()
    ));
    for (property_name, property_expr) in fn_invoke.arguments {
        let valid_rust_property_name = escape_rust_name(property_name.as_str())
            .from_case(Case::Camel)
            .to_case(Case::Snake);
        str.push_str(&format!(
            r#"
           .{}({})
        "#,
            valid_rust_property_name,
            generate_expression(property_expr)
        ));
    }
    str.push_str(
        r#"
            .build_struct(),
    );
    "#,
    );
    str
}

pub fn generate_resource(name: String, resource: Resource) -> String {
    let mut str = String::new();
    str.push_str(&format!(
        r#"
    let {} = {}::create(
        "{}",
        {}Args::builder()
        "#,
        name,
        resource.type_.get_rust_function_name(),
        name,
        resource.type_.get_rust_struct_name()
    ));
    for (property_name, property_expr) in resource.properties {
        let valid_rust_property_name = escape_rust_name(property_name.as_str())
            .from_case(Case::Camel)
            .to_case(Case::Snake);
        str.push_str(&format!(
            r#"
           .{}({})
        "#,
            valid_rust_property_name,
            generate_expression(property_expr)
        ));
    }
    str.push_str(
        r#"
            .build_struct(),
    );
    "#,
    );
    str
}

fn generate_object(element_id: ElementId, expr: BTreeMap<String, Expression>) -> String {
    let mut str = String::new();
    str.push_str(&format!(
        r#"
        {}::builder()
        "#,
        element_id.get_rust_struct_name()
    ));
    for (property_name, property_expr) in expr {
        str.push_str(&format!(
            r#"
           .{}({})
        "#,
            property_name,
            generate_expression(property_expr)
        ));
    }
    // for input in resource.inputs.clone() {
    //     str.push_str(&generate_resource_input(&input))
    // }
    str.push_str(
        r#"
            .build_struct()
    "#,
    );
    str
}

fn generate_expression(expr: Expression) -> String {
    match expr {
        Expression::String(s) => {
            let lit_str = LitStr::new(&s, proc_macro2::Span::call_site());
            format!("{}", lit_str.to_token_stream())
        }
        Expression::Number(n) => format!("{}", n),
        Expression::Integer(i) => format!("{}", i),
        Expression::Boolean(b) => format!("{}", b),
        Expression::Object(element_id, expr) => generate_object(element_id, expr),
        Expression::HashMap(map) => {
            let mut str = String::new();
            str.push_str("HashMap::new([");

            for (key, value) in map {
                str.push_str(format!("({}, {}),", key, generate_expression(value)).as_str());
            }

            str.push_str("])");
            str
        }
        Expression::Array(arr) => {
            let mut str = String::new();
            str.push_str("vec![");

            for value in arr {
                str.push_str(format!("{},", generate_expression(value)).as_str());
            }

            str.push(']');
            str
        }
    }
}
