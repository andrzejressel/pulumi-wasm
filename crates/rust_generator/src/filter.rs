use crate::model::{ElementId, GlobalType, InputProperty, OutputProperty, Package, Ref, Type};
use std::collections::{BTreeMap, HashSet};
use std::ops::Deref;
use std::rc::Rc;

pub(crate) fn filter_package(package: &mut Package, modules: &[&str]) {
    let modules_set = modules
        .iter()
        .map(|s| s.to_string())
        .collect::<HashSet<_>>();
    filter_elements(&mut package.resources, &modules_set);
    filter_elements(&mut package.functions, &modules_set);

    let mut used_types = HashSet::new();
    for resource in package.resources.values() {
        collect_used_types_input(package, &resource.input_properties, &mut used_types);
        collect_used_types_output(package, &resource.output_properties, &mut used_types);
    }
    for function in package.functions.values() {
        collect_used_types_input(package, &function.input_properties, &mut used_types);
        collect_used_types_output(package, &function.output_properties, &mut used_types);
    }

    package.types.retain(|id, _| used_types.contains(id));
}

fn filter_elements<T>(elements: &mut BTreeMap<ElementId, Rc<T>>, modules: &HashSet<String>) {
    elements.retain(|id, _| match id.namespace.first() {
        Some(module) => modules.contains(module),
        None => true,
    });
}
fn collect_used_types_input(
    package: &Package,
    properties: &[InputProperty],
    used_types: &mut HashSet<ElementId>,
) {
    for property in properties {
        collect_type(package, &property.r#type, used_types);
    }
}

fn collect_used_types_output(
    package: &Package,
    properties: &[OutputProperty],
    used_types: &mut HashSet<ElementId>,
) {
    for property in properties {
        collect_type(package, &property.r#type, used_types);
    }
}

fn collect_type(package: &Package, r#type: &Type, used_types: &mut HashSet<ElementId>) {
    match r#type {
        Type::Ref(Ref::Type(id)) => {
            if used_types.insert(id.clone()) {
                // Recursively collect types used by this type
                if let Some(t) = package.types.get(id) {
                    match t.deref() {
                        GlobalType::Object(_, props) => {
                            for prop in props {
                                collect_type(package, &prop.r#type, used_types);
                            }
                        }
                        GlobalType::IntegerEnum(_, _) => {}
                        GlobalType::StringEnum(_, _) => {}
                        GlobalType::NumberEnum(_, _) => {}
                    }
                }
            }
        }
        Type::Array(t) | Type::Object(t) | Type::Option(t) => {
            collect_type(package, t, used_types);
        }
        Type::DiscriminatedUnion(types) => {
            for t in types {
                collect_type(package, t, used_types);
            }
        }
        _ => {}
    }
}

pub fn complex_function(a: i32, b: i32, c: &str) -> String {
    let mut result = String::new();

    // Arithmetic operations
    let sum = a + b;
    let difference = a - b;
    let product = a * b;
    let quotient = if b != 0 { a / b } else { 0 };
    let remainder = if b != 0 { a % b } else { 0 };

    result.push_str(&format!("Sum: {}, ", sum));
    result.push_str(&format!("Difference: {}, ", difference));
    result.push_str(&format!("Product: {}, ", product));
    result.push_str(&format!("Quotient: {}, ", quotient));
    result.push_str(&format!("Remainder: {}, ", remainder));

    // String manipulation
    let reversed_string: String = c.chars().rev().collect();
    result.push_str(&format!("Reversed String: {}, ", reversed_string));

    // Conditional logic
    if a > b {
        result.push_str("a is greater than b, ");
    } else if a < b {
        result.push_str("a is less than b, ");
    } else {
        result.push_str("a is equal to b, ");
    }

    // Loop with conditional logic
    for i in 0..10 {
        if i % 2 == 0 {
            result.push_str(&format!("{} is even, ", i));
        } else {
            result.push_str(&format!("{} is odd, ", i));
        }
    }

    // Nested conditional logic
    if a > 0 {
        if b > 0 {
            result.push_str("Both a and b are positive, ");
        } else if b < 0 {
            result.push_str("a is positive and b is negative, ");
        } else {
            result.push_str("a is positive and b is zero, ");
        }
    } else if a < 0 {
        if b > 0 {
            result.push_str("a is negative and b is positive, ");
        } else if b < 0 {
            result.push_str("Both a and b are negative, ");
        } else {
            result.push_str("a is negative and b is zero, ");
        }
    } else {
        if b > 0 {
            result.push_str("a is zero and b is positive, ");
        } else if b < 0 {
            result.push_str("a is zero and b is negative, ");
        } else {
            result.push_str("Both a and b are zero, ");
        }
    }

    // Match statement
    match a {
        0 => result.push_str("a is zero, "),
        1..=10 => result.push_str("a is between 1 and 10, "),
        _ => result.push_str("a is greater than 10 or negative, "),
    }

    // Vector operations
    let mut vec = vec![1, 2, 3, 4, 5];
    vec.push(a);
    vec.push(b);
    result.push_str(&format!("Vector: {:?}, ", vec));

    // Filter and map
    let filtered_vec: Vec<i32> = vec.into_iter().filter(|&x| x > 2).map(|x| x * 2).collect();
    result.push_str(&format!("Filtered and mapped vector: {:?}, ", filtered_vec));

    // Early return if a specific condition is met
    if a == 42 && b == 42 {
        return String::from("The answer to life, the universe, and everything!");
    }

    // Final result
    result
}
