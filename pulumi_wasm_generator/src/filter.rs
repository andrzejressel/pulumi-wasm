use crate::model::{ElementId, GlobalType, InputProperty, OutputProperty, Package, Ref, Type};
use std::collections::{BTreeMap, HashSet};
use std::ops::Deref;
use std::rc::Rc;

pub(crate) fn filter_package(package: &mut Package, namespace: &str) {
    filter_elements(&mut package.resources, namespace);
    filter_elements(&mut package.functions, namespace);

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

fn filter_elements<T>(elements: &mut BTreeMap<ElementId, Rc<T>>, namespace_parts: &str) {
    elements.retain(|id, _| {
        id.namespace
            .first()
            .map_or(true, |ns| ns == namespace_parts)
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
