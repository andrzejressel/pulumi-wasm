use crate::code_generation::yaml::model::Variable::FnInvokeVariable;
use crate::code_generation::yaml::yaml_model::{
    YamlExpression, YamlFile, YamlFnInvoke, YamlResource, YamlVariable,
};
use crate::model::{ElementId, GlobalType, Package, Ref, Type};
use anyhow::Result;
use anyhow::{anyhow, Context};
use std::collections::BTreeMap;

pub(crate) fn yaml_to_model(yaml_file: YamlFile, package: &Package) -> Result<Example> {
    let mut resources = BTreeMap::new();

    for (name, yaml_resource) in yaml_file.resources {
        let resource = map_resource(&yaml_resource, package).with_context(|| {
            format!(
                "Failed to map YAML resource name [{}] value [{:?}]",
                name, yaml_resource
            )
        })?;
        resources.insert(name, resource);
    }

    let mut variables = BTreeMap::new();
    for (name, yaml_variable) in yaml_file.variables {
        let variable = map_variable(&yaml_variable, package).with_context(|| {
            format!(
                "Failed to map YAML variable name [{}] value [{:?}]",
                name, yaml_variable
            )
        })?;
        variables.insert(name, variable);
    }

    Ok(Example {
        resources,
        variables,
    })
}

#[derive(Debug, PartialEq)]
pub(crate) struct Example {
    pub(crate) resources: BTreeMap<String, Resource>,
    pub(crate) variables: BTreeMap<String, Variable>,
}

#[derive(Debug, PartialEq)]
pub(crate) struct Resource {
    pub(crate) type_: ElementId,
    pub(crate) name: Option<String>,
    pub(crate) properties: BTreeMap<String, Expression>,
}

#[derive(Debug, PartialEq)]
pub(crate) enum Variable {
    FnInvokeVariable(FnInvoke),
}

#[derive(Debug, PartialEq)]
pub(crate) struct FnInvoke {
    pub(crate) function: ElementId,
    pub(crate) arguments: BTreeMap<String, Expression>,
}

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

fn map_resource(yaml_resource: &YamlResource, context: &Package) -> Result<Resource> {
    let resource = context
        .resource_name_map
        .get(&yaml_resource.type_)
        .with_context(|| format!("resource type not found: {}", yaml_resource.type_))?;

    let mut properties = BTreeMap::new();

    for (argument_name, argument_value) in &yaml_resource.properties {
        let resource_argument = &resource
            .input_properties
            .iter()
            .find(|k| k.name == *argument_name)
            .with_context(|| format!("argument not found: {}", argument_name))?;

        let type_without_option = remove_option(&resource_argument.r#type).with_context(|| {
            format!(
                "Cannot remove option from argument [{}] type [{:?}]",
                argument_name, argument_value
            )
        })?;

        let expression = map_expression(context, &type_without_option, argument_value)
            .with_context(|| {
                format!(
                    "Failed to map argument name [{}] value [{:?}]",
                    argument_name, argument_value
                )
            })?;

        properties.insert(argument_name.clone(), expression);
    }

    Ok(Resource {
        type_: resource.element_id.clone(),
        name: yaml_resource.name.clone(),
        properties,
    })
}

fn map_variable(yaml_variable: &YamlVariable, context: &Package) -> Result<Variable> {
    Ok(FnInvokeVariable(
        map_fn_invoke(&yaml_variable.fn_invoke, context)
            .with_context(|| format!("Failed to map yaml variable [{:?}]", yaml_variable))?,
    ))
}

fn map_fn_invoke(yaml_fn_invoke: &YamlFnInvoke, context: &Package) -> Result<FnInvoke> {
    let function = context
        .function_name_map
        .get(&yaml_fn_invoke.function)
        .with_context(|| format!("function not found: {}", yaml_fn_invoke.function))?;

    let mut arguments = BTreeMap::new();

    for (argument_name, argument_value) in &yaml_fn_invoke.arguments {
        let function_argument = &function
            .input_properties
            .iter()
            .find(|k| k.name == *argument_name)
            .with_context(|| format!("argument not found: {}", argument_name))?;

        let type_without_option = remove_option(&function_argument.r#type).with_context(|| {
            format!(
                "Cannot remove option from argument [{}] type [{:?}]",
                argument_name, argument_value
            )
        })?;

        let expression = map_expression(context, &type_without_option, argument_value)
            .with_context(|| {
                format!(
                    "Failed to map argument name [{}] value [{:?}]",
                    argument_name, argument_value
                )
            })?;

        arguments.insert(argument_name.clone(), expression);
    }

    Ok(FnInvoke {
        function: function.element_id.clone(),
        arguments,
    })
}

fn map_array(
    context: &Package,
    type_without_option: &TypeWithoutOption,
    yaml_expressions: &Vec<YamlExpression>,
) -> Result<Expression> {
    let mut expressions = Vec::new();

    for expression in yaml_expressions {
        let mapped_expression = map_expression(context, type_without_option, expression)
            .with_context(|| format!("Failed to map yaml expression [{:?}]", expression))?;
        expressions.push(mapped_expression);
    }

    Ok(Expression::Array(expressions))
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

fn remove_option(type_: &Type) -> Result<TypeWithoutOption> {
    match type_ {
        Type::Boolean => Ok(TypeWithoutOption::Boolean),
        Type::Integer => Ok(TypeWithoutOption::Integer),
        Type::Number => Ok(TypeWithoutOption::Number),
        Type::String => Ok(TypeWithoutOption::String),
        Type::ConstString(_) => Ok(TypeWithoutOption::String),
        Type::Array(a) => Ok(TypeWithoutOption::Array(Box::new(remove_option(a)?))),
        Type::Object(o) => Ok(TypeWithoutOption::Object(Box::new(remove_option(o)?))),
        Type::Ref(r) => Ok(TypeWithoutOption::Ref(r.clone())),
        Type::Option(o) => remove_option(o),
        Type::DiscriminatedUnion(_) => Err(anyhow!("Discriminated union are not supported")),
    }
}

fn map_expression(
    package_context: &Package,
    type_without_option: &TypeWithoutOption,
    yaml_expression: &YamlExpression,
) -> Result<Expression> {
    if let YamlExpression::Object(map) = &yaml_expression {
        if map.len() == 1 {
            let key = map.keys().next().unwrap();
            if key.starts_with("fn::") {
                return Err(anyhow!("fn:: are not supported"));
            }
        }
    }

    match (type_without_option, yaml_expression) {
        (TypeWithoutOption::String, YamlExpression::String(value)) => {
            Ok(Expression::String(value.clone()))
        }
        (TypeWithoutOption::Boolean, YamlExpression::Boolean(value)) => {
            Ok(Expression::Boolean(*value))
        }
        (TypeWithoutOption::Array(arr), YamlExpression::Array(expression)) => {
            map_array(package_context, arr, expression).with_context(|| {
                format!(
                    "Failed to map array type [{:?}] with expression [{:?}]",
                    arr, expression
                )
            })
        }
        (TypeWithoutOption::Ref(r), YamlExpression::Object(properties)) => {
            map_type(package_context, r, properties)
        }
        (TypeWithoutOption::Integer, YamlExpression::Number(f)) => {
            Ok(Expression::Integer(f.round() as i64))
        }
        (TypeWithoutOption::Number, YamlExpression::Number(f)) => Ok(Expression::Number(*f)),
        (a, b) => Err(anyhow!("Invalid type combination: {:?} with {:?}", a, b)),
    }
}

fn map_type(
    context: &Package,
    ref_: &Ref,
    properties: &BTreeMap<String, YamlExpression>,
) -> Result<Expression> {
    let element_id = match ref_ {
        Ref::Type(element_id) => element_id,
        Ref::Archive => return Err(anyhow!("Archive ref is not supported")),
        Ref::Asset => return Err(anyhow!("Asset ref is not supported")),
        Ref::Any => return Err(anyhow!("Any ref is not supported")),
    };

    let tpe = &context.types[element_id];

    let gtp = match tpe {
        GlobalType::Object(_, gtp) => gtp,
        GlobalType::NumberEnum(_, _) => return Err(anyhow!("NumberEnum type is not supported")),
        GlobalType::IntegerEnum(_, _) => return Err(anyhow!("IntegerEnum type is not supported")),
        GlobalType::StringEnum(_, _) => return Err(anyhow!("StringEnum type is not supported")),
    };

    let mut new_properties = BTreeMap::new();

    for (property_name, property_value) in properties {
        let field = gtp
            .iter()
            .find(|f| f.name == *property_name)
            .with_context(|| format!("property not found: {}", property_name))?;

        let type_without_option = remove_option(&field.r#type).with_context(|| {
            format!(
                "Cannot remove option from argument [{}] type [{:?}]",
                property_name, property_value
            )
        })?;

        let expression = map_expression(context, &type_without_option, property_value)
            .with_context(|| {
                format!(
                    "Failed to map proparty name [{}] value [{:?}]",
                    property_name, property_value
                )
            })?;

        new_properties.insert(property_name.clone(), expression);
    }

    Ok(Expression::Object(element_id.clone(), new_properties))
}
