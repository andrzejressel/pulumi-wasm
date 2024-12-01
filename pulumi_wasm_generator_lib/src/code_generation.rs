use crate::model::{ElementId, Ref};
use crate::yaml::model::{yaml_to_model, Example, Expression, Resource};
use crate::yaml::yaml_model::YamlFile;
use std::collections::{BTreeMap, HashMap};
use std::fmt::format;

pub fn generate_code_from_string(yaml: String, package: &crate::model::Package) -> String {
    let yaml_file = YamlFile::from_yaml(yaml.as_str()).unwrap();
    let example = yaml_to_model(yaml_file, package.name.clone(), package);
    generate_code(example)
}

pub fn generate_code(example: Example) -> String {
    let mut result = r"
use pulumi_wasm_rust::Output;
use pulumi_wasm_rust::{add_export, pulumi_main};
#[pulumi_main]
fn test_main() -> Result<(), Error> {
    "
    .trim()
    .to_string();
    result.push_str("\n");

    for (name, resource) in example.resources {
        result.push_str(generate_resource(name, resource).as_str());
        result.push('\n');
    }

    result.push_str("}");

    println!("{}", result);
    let syntax_tree = syn::parse_file(result.as_str()).unwrap();
    let formatted = prettyplease::unparse(&syntax_tree);
    formatted
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
        str.push_str(&format!(
            r#"
           .{}({})
        "#,
            property_name,
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

            str.push_str("]");
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
        let code = generate_code(model);
        assert_eq!(example_array::get_rust_code(), code)
    }

    #[test]
    fn test_access_rule() {
        let model = access_rule::get_model();
        let code = generate_code(model);
        assert_eq!(access_rule::get_rust_code(), code)
    }
}
