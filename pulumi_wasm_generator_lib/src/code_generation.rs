use crate::model::ElementId;
use crate::utils::escape_rust_name;
use crate::yaml::model::{yaml_to_model, Example, Expression, Resource};
use crate::yaml::yaml_model::YamlFile;
use anyhow::{anyhow, Context, Result};
use convert_case::Case;
use convert_case::Casing;
use std::collections::BTreeMap;
use std::panic;

pub fn generate_code_from_string(yaml: String, package: &crate::model::Package) -> Result<String> {
    let yaml_file =
        YamlFile::from_yaml(yaml.as_str()).context(format!("Failed to parse YAML: {}", yaml))?;
    let example = panic::catch_unwind(|| yaml_to_model(yaml_file, package.name.clone(), package))
        .map_err(|_| anyhow!("Failed to convert YAML to model"))
        .context(format!("Failed to convert YAML {} to model", yaml))?;
    generate_code(example)
}

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

    for (name, resource) in example.resources {
        result.push_str(generate_resource(name, resource).as_str());
        result.push('\n');
    }

    result.push('}');

    let syntax_tree = syn::parse_file(result.as_str())
        .context(format!("Failed to parse generated Rust code:\n{}", result))?;
    let formatted = prettyplease::unparse(&syntax_tree);
    Ok(formatted)
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
        Expression::String(s) => format!("\"{}\"", s),
        Expression::Number(n) => format!("{}", n),
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

#[cfg(test)]
mod tests {
    use crate::code_generation::generate_code;
    use crate::yaml::tests::{access_rule, example_array};

    #[test]
    fn test_example_array() {
        let model = example_array::get_model();
        let code = generate_code(model).unwrap();
        assert_eq!(example_array::get_rust_code(), code)
    }

    #[test]
    fn test_access_rule() {
        let model = access_rule::get_model();
        let code = generate_code(model).unwrap();
        assert_eq!(access_rule::get_rust_code(), code)
    }
}
