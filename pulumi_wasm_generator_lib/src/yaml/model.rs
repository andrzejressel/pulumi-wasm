use crate::model::{ElementId, GlobalType, Package, Ref, Type};
use crate::yaml::yaml_model::{YamlExpression, YamlFile, YamlResource};
use std::collections::{BTreeMap, HashMap};

struct PackageContext<'a> {
    package: &'a Package,
    resource_name_map: HashMap<String, &'a crate::model::Resource>,
}

pub(crate) fn yaml_to_model(
    yaml_file: YamlFile,
    provider_name: String,
    package: &Package,
) -> Example {
    let mut resource_name_map = HashMap::new();

    for (element_id, resource) in &package.resources {
        let mut chunks = Vec::new();
        chunks.push(provider_name.clone());
        chunks.extend(element_id.namespace.clone());
        chunks.push(element_id.name.clone());

        let name = chunks.join(":");

        resource_name_map.insert(name.clone(), resource);
    }

    let context = PackageContext {
        package,
        resource_name_map,
    };

    let mut resources = BTreeMap::new();

    for (name, yaml_resource) in yaml_file.resources {
        let resource = map_resource(yaml_resource, &context);
        resources.insert(name, resource);
    }

    Example { resources }
}

#[derive(Debug, PartialEq)]
pub(crate) struct Example {
    pub(crate) resources: BTreeMap<String, Resource>,
}

#[derive(Debug, PartialEq)]
pub(crate) struct Resource {
    pub(crate) type_: ElementId,
    pub(crate) name: Option<String>,
    pub(crate) properties: BTreeMap<String, Expression>,
}

struct Variable {}

#[derive(Debug, PartialEq)]
pub(crate) enum Expression {
    String(String),
    Integer(i64),
    Number(f64),
    Boolean(bool),
    Object(ElementId, BTreeMap<String, Expression>),
    HashMap(BTreeMap<String, Expression>),
    Array(Vec<Expression>),
}

fn map_resource(yaml_resource: YamlResource, context: &PackageContext) -> Resource {
    let resource = context
        .resource_name_map
        .get(&yaml_resource.type_)
        .unwrap_or_else(|| panic!("resource type not found: {}", yaml_resource.type_));

    let mut properties = BTreeMap::new();

    for (argument_name, argument_value) in &yaml_resource.properties {
        let resource_argument = &resource
            .input_properties
            .iter()
            .find(|k| k.name == *argument_name)
            .unwrap_or_else(|| panic!("argument not found: {}", argument_name));

        let type_without_option = remove_option(&resource_argument.r#type);

        properties.insert(
            argument_name.clone(),
            map_expression(context, &type_without_option, argument_value),
        );
    }

    Resource {
        type_: context.resource_name_map[&yaml_resource.type_]
            .element_id
            .clone(),
        name: yaml_resource.name.clone(),
        properties,
    }
}

fn map_array(
    context: &PackageContext,
    type_without_option: &Box<TypeWithoutOption>,
    yaml_expressions: &Vec<YamlExpression>,
) -> Expression {
    let mut expressions = Vec::new();

    for expression in yaml_expressions {
        expressions.push(map_expression(context, type_without_option, expression));
    }

    Expression::Array(expressions)
}

#[derive(Debug)]
enum TypeWithoutOption {
    Boolean,
    Integer,
    Number,
    String,
    Array(Box<TypeWithoutOption>),
    Object(Box<TypeWithoutOption>),
    Ref(Ref),
}

fn remove_option(type_: &Type) -> TypeWithoutOption {
    match type_ {
        Type::Boolean => TypeWithoutOption::Boolean,
        Type::Integer => TypeWithoutOption::Integer,
        Type::Number => TypeWithoutOption::Number,
        Type::String => TypeWithoutOption::String,
        Type::Array(a) => TypeWithoutOption::Array(Box::new(remove_option(a))),
        Type::Object(o) => TypeWithoutOption::Object(Box::new(remove_option(o))),
        Type::Ref(r) => TypeWithoutOption::Ref(r.clone()),
        Type::Option(o) => remove_option(o),
    }
}

fn map_expression(
    package_context: &PackageContext,
    type_without_option: &TypeWithoutOption,
    yaml_expression: &YamlExpression,
) -> Expression {
    if let YamlExpression::Object(map) = &yaml_expression {
        if map.len() == 1 {
            let key = map.keys().next().unwrap();
            if key.starts_with("fn::") {
                panic!("fn:: are not supported")
            }
        }
    }

    match (type_without_option, yaml_expression) {
        (TypeWithoutOption::String, YamlExpression::String(value)) => {
            Expression::String(value.clone())
        }
        (TypeWithoutOption::Boolean, YamlExpression::Boolean(value)) => Expression::Boolean(*value),
        (TypeWithoutOption::Array(arr), YamlExpression::Array(expression)) => {
            map_array(package_context, arr, expression)
        }
        (TypeWithoutOption::Ref(r), YamlExpression::Object(properties)) => {
            map_type(package_context, r, properties)
        }
        (TypeWithoutOption::Integer, YamlExpression::Number(f)) => {
            Expression::Integer(f.round() as i64)
        }
        (TypeWithoutOption::Number, YamlExpression::Number(f)) => Expression::Number(*f),
        (a, b) => panic!("Invalid type combination: {:?} with {:?}", a, b),
    }
}

fn map_type(
    context: &PackageContext,
    ref_: &Ref,
    properties: &BTreeMap<String, YamlExpression>,
) -> Expression {
    let element_id = match ref_ {
        Ref::Type(element_id) => element_id,
        Ref::Archive => panic!("Archive ref is not supported"),
        Ref::Asset => panic!("Asset ref is not supported"),
        Ref::Any => panic!("Any ref is not supported"),
    };

    let tpe = &context.package.types[element_id];

    let gtp = match tpe {
        GlobalType::Object(gtp) => gtp,
        GlobalType::String => panic!("String type is not supported"),
        GlobalType::Boolean => panic!("Boolean type is not supported"),
        GlobalType::Number => panic!("Number type is not supported"),
        GlobalType::Integer => panic!("Integer type is not supported"),
    };

    let mut new_properties = BTreeMap::new();

    for (property_name, property_value) in properties {
        let field = gtp
            .iter()
            .find(|f| f.name == *property_name)
            .unwrap_or_else(|| panic!("property not found: {}", property_name));
        let type_without_option = remove_option(&field.r#type);
        // map_expression(&type_without_option, property_value);
        new_properties.insert(
            property_name.clone(),
            map_expression(context, &type_without_option, property_value),
        );
    }

    Expression::Object(element_id.clone(), new_properties)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::yaml::tests::{
        access_rule, example_access_organization, example_array, example_numbers,
    };
    use crate::{extract_schema_from_file, schema};

    #[test]
    fn test_map_expression() {
        let schema_package: schema::Package =
            extract_schema_from_file("test_cases/cloudflare.json".as_ref()).unwrap();
        let package = schema::to_model(&schema_package).unwrap();
        let yaml_file = example_access_organization::get_yaml_file();

        yaml_to_model(yaml_file, "cloudflare".to_string(), &package);
    }

    #[test]
    fn test_access_rule() {
        let schema_package: schema::Package =
            extract_schema_from_file("test_cases/cloudflare.json".as_ref()).unwrap();
        let package = schema::to_model(&schema_package).unwrap();
        let yaml_file = access_rule::get_yaml_file();

        let result = yaml_to_model(yaml_file, "cloudflare".to_string(), &package);

        assert_eq!(result, access_rule::get_model());
    }

    #[test]
    fn test_example_array() {
        let schema_package: schema::Package =
            extract_schema_from_file("test_cases/cloudflare.json".as_ref()).unwrap();
        let package = schema::to_model(&schema_package).unwrap();
        let yaml_file = example_array::get_yaml_file();

        let result = yaml_to_model(yaml_file, "cloudflare".to_string(), &package);

        assert_eq!(result, example_array::get_model());
    }

    #[test]
    fn test_example_numbers() {
        let schema_package: schema::Package =
            extract_schema_from_file("test_cases/cloudflare.json".as_ref()).unwrap();
        let package = schema::to_model(&schema_package).unwrap();
        let yaml_file = example_numbers::get_yaml_file();

        let result = yaml_to_model(yaml_file, "cloudflare".to_string(), &package);

        assert_eq!(result, example_numbers::get_model());
    }
}