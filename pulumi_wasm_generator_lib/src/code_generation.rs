use crate::model::ElementId;
use anyhow::Result;
use base64::prelude::*;
use prost::Message;
use pulumi_wasm_proto::codegen::literal_value_expression;
use pulumi_wasm_proto::codegen::node::Value;
use pulumi_wasm_proto::codegen::{
    expression, traverser, ScopeTraversalExpression, Traversal, Traverser,
};
use pulumi_wasm_proto::codegen::{Expression, Node, PclProtobufProgram, Resource, ResourceInput};
use std::ops::{Add, Deref};

pub fn generate_example(example: String) -> Result<String> {
    let mut arr = BASE64_STANDARD.decode(example)?;

    let program = PclProtobufProgram::decode(&*arr)?;

    let mut result = r"
use pulumi_wasm_rust::Output;
use pulumi_wasm_rust::{add_export, pulumi_main};

#[pulumi_main]
fn test_main() -> Result<(), Error> {
    "
    .trim()
    .to_string();
    result.push_str("\n");

    for (node) in program.nodes {
        println!("{:?}", node);
        let str = generate_node(&node);
        result.push_str(&str);
    }

    result.push_str("}");

    Ok(result)
}

fn generate_node(node: &Node) -> String {
    let value = &node.value;

    return match value {
        None => todo!(),
        Some(a) => match a {
            Value::Resource(r) => generate_resource(r),
            Value::LocalVariable(ll) => todo!(),
            Value::ConfigVariable(cv) => todo!(),
            Value::OutputVariable(ov) => todo!(),
        },
    };

    // match node.value {
    //     None => panic!("No value found"),
    //     Some(a) => match a {
    //         Value::Resource(_) => {}
    //         Value::LocalVariable(_) => {}
    //         Value::ConfigVariable(_) => {}
    //         Value::OutputVariable(_) => {}
    //     }
    // }
}

fn generate_resource(resource: &Resource) -> String {
    let mut str = String::new();

    let element = ElementId::new(&*resource.token).unwrap();
    println!("{:?}", element);

    str.push_str(&format!(
        r#"
    let {} = {}::create(
        "{}",
        {}Args::builder()
        "#,
        resource.name,
        element.get_rust_function_name(),
        resource.name,
        element.get_rust_struct_name()
    ));

    for input in resource.inputs.clone() {
        str.push_str(&generate_resource_input(&input))
    }

    str.push_str(
        r#"
            .build_struct(),
    );
    "#,
    );
    str
}

fn generate_resource_input(resource_input: &ResourceInput) -> String {
    format!(
        ".{}({})",
        resource_input.name,
        generate_expression(&resource_input.value)
    )
}

fn generate_expression(expression: &Option<Expression>) -> String {
    match expression {
        None => todo!(),
        Some(e) => match &e.value {
            None => todo!(),
            Some(ee) => match ee {
                expression::Value::LiteralValueExpression(v) => match &v.value {
                    None => todo!(),
                    Some(v) => match v {
                        literal_value_expression::Value::UnknownValue(_) => todo!("Unknown value"),
                        literal_value_expression::Value::StringValue(s) => {
                            format!("\"{}\"", s)
                        }
                        literal_value_expression::Value::NumberValue(n) => n.to_string(),
                        literal_value_expression::Value::BoolValue(b) => match b {
                            true => "true".to_string(),
                            false => "false".to_string(),
                        },
                    },
                },
                expression::Value::TemplateExpression(_) => todo!("Template expression"),
                expression::Value::IndexExpression(i) => format!(
                    "{}[{}]",
                    generate_expression_box(&i.as_ref().collection),
                    generate_expression_box(&i.key)
                ),
                expression::Value::ObjectConsExpression(_) => todo!("Object cons expression"),
                expression::Value::TupleConsExpression(t) => t
                    .items
                    .iter()
                    .map(|i| generate_expression(&Some(i.clone())))
                    .collect::<Vec<_>>()
                    .join("."),
                expression::Value::FunctionCallExpression(_) => todo!("Function call expression"),
                expression::Value::RelativeTraversalExpression(_) => {
                    todo!("Relative traversal expression")
                }
                expression::Value::ScopeTraversalExpression(ste) => {
                    generate_scope_traversal_expression(ste)
                }
                expression::Value::AnonymousFunctionExpression(_) => {
                    todo!("Anonymous function expression")
                }
                expression::Value::ConditionalExpression(_) => todo!("Conditional expression"),
                expression::Value::BinaryOpExpression(_) => todo!("Binary op expression"),
                expression::Value::UnaryOpExpression(_) => todo!("Unary op expression"),
            },
        },
    }
}

fn generate_scope_traversal_expression(ste: &ScopeTraversalExpression) -> String {
    let traversals = match &ste.traversal {
        None => todo!(),
        Some(t) => generate_traversal(t),
    };

    // let mut vec = vec![ste.root_name.clone()];
    // vec.extend(traversals);

    traversals.join(".")

    // ste.root_name + match ste.traversal {
    //     None => {}
    //     Some(t) =>
    // }
}

fn generate_traversal(t: &Traversal) -> Vec<String> {
    t
        .each
        .iter()
        .map(|i| generate_traverser(i).clone())
        .flatten()
        .map(|s| s.clone())
        .collect::<Vec<_>>()
}

fn generate_traverser(t: &Traverser) -> Vec<String> {
    match &t.value {
        None => {
            todo!("No value found")
        }
        Some(v) => match v {
            traverser::Value::TraverseAttr(attr) => vec![attr.name.clone()],
            traverser::Value::TraverseIndex(index) => todo!("Index"),
            traverser::Value::TraverseRoot(root) => {vec![root.name.clone()]},
            traverser::Value::TraverseSplat(splat) => {
                let v = splat.clone().each.unwrap().clone();
                generate_traversal(&v)
            }
        },
    }
}

fn generate_expression_box(expression: &Option<Box<Expression>>) -> String {
    match expression {
        None => generate_expression(&None),
        Some(a) => generate_expression(&Some(a.deref().clone())), // FIXME
    }
}
