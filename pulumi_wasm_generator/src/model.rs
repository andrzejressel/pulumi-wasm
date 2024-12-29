use crate::utils::{escape_rust_name, replace_multiple_dashes};
use anyhow::{Context, Result};
use convert_case::Case;
use convert_case::Case::UpperCamel;
use convert_case::Casing;
use std::collections::BTreeMap;

#[derive(Clone, Debug, PartialEq, Hash, Ord, PartialOrd, Eq)]
pub(crate) enum Type {
    Boolean,
    Integer,
    Number,
    String,
    Array(Box<Type>),
    Object(Box<Type>),
    Ref(Ref),
    Option(Box<Type>),
    DiscriminatedUnion(Vec<Type>),
    ConstString(String),
}

impl Type {
    pub(crate) fn get_rust_type(&self, depth: usize) -> String {
        match self {
            Type::Boolean => "bool".into(),
            Type::Integer => "i32".into(),
            Type::Number => "f64".into(),
            Type::String => "String".into(),
            Type::Array(type_) => {
                format!("Vec<{}>", type_.get_rust_type(depth))
            }
            Type::Object(type_) => {
                format!(
                    "std::collections::HashMap<String, {}>",
                    type_.get_rust_type(depth)
                )
            }
            Type::Ref(r) => match r {
                Ref::Type(tpe) => {
                    let prefix = if depth > 0 {
                        "super::".repeat(depth)
                    } else {
                        "self::".to_string()
                    };
                    format!("{}types::{}", prefix, tpe.get_rust_absolute_name())
                }
                Ref::Archive => "String".to_string(), //FIXME
                Ref::Asset => "String".to_string(),   //FIXME
                Ref::Any => "String".to_string(),     //FIXME
            },
            Type::Option(type_) => format!("Option<{}>", type_.get_rust_type(depth)),
            Type::DiscriminatedUnion(refs) => format!(
                "pulumi_wasm_rust::OneOf{}<{}>",
                refs.len(),
                refs.iter()
                    .map(|r| r.get_rust_type(depth))
                    .collect::<Vec<_>>()
                    .join(", ")
            ),
            Type::ConstString(s) => {
                let prefix = if depth > 0 {
                    "super::".repeat(depth)
                } else {
                    "self::".to_string()
                };
                format!("{}constants::ConstString{}", prefix, s.to_case(UpperCamel)).to_string()
            }
        }
    }

    pub(crate) fn get_consts(&self) -> Vec<String> {
        match self {
            Type::Boolean => vec![],
            Type::Integer => vec![],
            Type::Number => vec![],
            Type::String => vec![],
            Type::ConstString(s) => vec![s.clone()],
            Type::Ref(_) => vec![],
            Type::Array(t) => t.get_consts(),
            Type::Object(o) => o.get_consts(),
            Type::Option(o) => o.get_consts(),
            Type::DiscriminatedUnion(_) => vec![],
        }
    }
}

#[derive(Debug, PartialEq, Hash, Ord, PartialOrd, Eq)]
pub(crate) struct InputProperty {
    pub(crate) name: String,
    pub(crate) r#type: Type,
    pub(crate) description: Option<String>,
}

impl InputProperty {
    pub(crate) fn get_rust_argument_name(&self) -> String {
        escape_rust_name(ElementId::create_valid_wit_rust_id(self.name.as_str()).as_str()).into()
    }
}

#[derive(Debug, PartialEq, Hash, Ord, PartialOrd, Eq)]
pub(crate) struct OutputProperty {
    pub(crate) name: String,
    pub(crate) r#type: Type,
    pub(crate) description: Option<String>,
}

impl OutputProperty {
    pub(crate) fn get_rust_argument_name(&self) -> String {
        escape_rust_name(ElementId::create_valid_wit_rust_id(self.name.as_str()).as_str()).into()
    }
}

#[derive(Debug, PartialEq, Hash, Ord, PartialOrd, Eq)]
pub(crate) struct GlobalTypeProperty {
    pub(crate) name: String,
    pub(crate) r#type: Type,
    pub(crate) description: Option<String>,
}

#[derive(Debug, PartialEq, PartialOrd)]
pub(crate) enum GlobalType {
    Object(Option<String>, Vec<GlobalTypeProperty>),
    StringEnum(Option<String>, Vec<StringEnumElement>),
    NumberEnum(Option<String>, Vec<NumberEnumElement>),
    IntegerEnum(Option<String>, Vec<IntegerEnumElement>),
}

#[derive(Debug, PartialEq, Hash, Ord, PartialOrd, Eq)]
pub(crate) struct StringEnumElement {
    pub(crate) name: String,
    pub(crate) value: Option<String>,
    pub(crate) description: Option<String>,
}

#[derive(Debug, PartialEq, PartialOrd)]
pub(crate) struct NumberEnumElement {
    pub(crate) name: String,
    pub(crate) value: f64,
    pub(crate) description: Option<String>,
}

#[derive(Debug, PartialEq, PartialOrd)]
pub(crate) struct IntegerEnumElement {
    pub(crate) name: String,
    pub(crate) value: i64,
    pub(crate) description: Option<String>,
}

#[derive(Debug, PartialEq, Hash, Ord, PartialOrd, Eq)]
pub(crate) struct Resource {
    pub(crate) element_id: ElementId,
    // pub(crate) name: String,
    pub(crate) description: Option<String>,
    pub(crate) input_properties: Vec<InputProperty>,
    pub(crate) output_properties: Vec<OutputProperty>,
}

#[derive(Debug, PartialEq, Hash, Ord, PartialOrd, Eq)]
pub(crate) struct Function {
    pub(crate) element_id: ElementId,
    // pub(crate) name: String,
    pub(crate) description: Option<String>,
    pub(crate) input_properties: Vec<InputProperty>,
    pub(crate) output_properties: Vec<OutputProperty>,
}

#[derive(Debug, PartialEq)]
pub(crate) struct Package {
    pub(crate) name: String,
    pub(crate) display_name: Option<String>,
    pub(crate) version: String,
    pub(crate) resources: BTreeMap<ElementId, Resource>,
    pub(crate) functions: BTreeMap<ElementId, Function>,
    pub(crate) types: BTreeMap<ElementId, GlobalType>,
}

#[derive(Clone, Eq, PartialEq, Hash, Ord, PartialOrd, Debug)]
pub(crate) enum Ref {
    Type(ElementId),
    Archive,
    Asset,
    Any,
}

#[derive(Clone, Debug, PartialEq, Hash, Ord, PartialOrd, Eq)]
pub(crate) struct ElementId {
    pub(crate) namespace: Vec<String>,
    pub(crate) name: String,
    pub(crate) raw: String,
}

impl ElementId {
    pub(crate) fn get_rust_struct_name(&self) -> String {
        self.name.clone().to_case(Case::Pascal)
    }

    pub(crate) fn get_rust_absolute_name(&self) -> String {
        let mut parts = self.namespace.clone();
        parts.push(self.name.clone().to_case(Case::Pascal));
        parts.join("::")
    }

    pub(crate) fn get_rust_function_name(&self) -> String {
        self.name.clone().from_case(UpperCamel).to_case(Case::Snake)
    }

    pub(crate) fn get_rust_package_name(&self) -> String {
        escape_rust_name(&self.name.clone().to_case(Case::Snake)).to_string()
    }

    pub(crate) fn get_rust_namespace_name(&self) -> String {
        let mut vec = self.namespace.clone();
        vec.push(self.name.clone());
        Self::create_valid_id(&vec.join("-"))
    }

    fn create_valid_wit_rust_id(s: &str) -> String {
        Self::create_valid_wit_id(s).replace("-", "_")
    }

    fn create_valid_wit_id(s: &str) -> String {
        let result: String = s
            .chars()
            .map(|c| {
                if c.is_uppercase() {
                    format!("-{}", c.to_lowercase())
                } else if !c.is_alphanumeric() {
                    "-".to_string()
                } else {
                    c.to_string()
                }
            })
            .collect();

        let result = replace_multiple_dashes(&result);
        let result = result.trim_matches('-').to_string();
        result
    }

    fn create_valid_id(s: &str) -> String {
        Self::create_valid_wit_id(s).replace('-', "_")
    }
}

impl Ref {
    pub(crate) fn new(raw: &str) -> Result<Self> {
        if raw == "pulumi.json#/Archive" {
            Ok(Ref::Archive)
        } else if raw == "pulumi.json#/Asset" {
            Ok(Ref::Asset)
        } else if raw == "pulumi.json#/Any" {
            Ok(Ref::Any)
        } else if raw.starts_with("#/types/") {
            Ok(Ref::Type(ElementId::new(
                raw.strip_prefix("#/types/")
                    .context(format!("Cannot strip types prefix from {raw}"))?,
            )?))
            // return Ok(Ref::Element(ElementId::new(raw)?));
        } else {
            Err(anyhow::anyhow!("Cannot generate ref from [{raw}]."))
        }
    }
}

impl ElementId {
    pub(crate) fn new(raw: &str) -> Result<Self> {
        let raw = raw.replace("%2F", "/");
        let parts: Vec<&str> = raw.split(':').collect();
        if parts.len() != 3 {
            return Err(anyhow::anyhow!("Cannot generate element id from [{raw}]"));
        }
        let name = parts[2].to_string();
        let namespace = parts[1].split('/').collect::<Vec<_>>();

        let final_namespaces = namespace
            .into_iter()
            .filter(|s| *s != "index" && s.to_lowercase() != name.to_lowercase() && !s.is_empty())
            .map(|s| s.replace("-", "_").to_string())
            .map(|s| escape_rust_name(&s).to_string())
            .collect::<Vec<_>>();

        Ok(ElementId {
            namespace: final_namespaces.iter().map(|s| s.to_string()).collect(),
            name,
            raw: raw.to_string(),
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::model::ElementId;

    #[test]
    fn extract_namespace_from_command() {
        let id = "command:remote:Connection";
        assert_eq!(
            ElementId::new(id).unwrap(),
            ElementId {
                namespace: vec!["remote".to_string()],
                name: "Connection".to_string(),
                raw: id.to_string(),
            }
        );
    }

    #[test]
    fn extract_namespace_from_random() {
        let id = "random:index/randomBytes:RandomBytes";
        assert_eq!(
            ElementId::new(id).unwrap(),
            ElementId {
                namespace: vec![],
                name: "RandomBytes".to_string(),
                raw: id.to_string(),
            }
        );
    }

    #[test]
    fn perform_escaping() {
        let id = "docker:index%2FContainerPort:ContainerPort";
        assert_eq!(
            ElementId::new(id).unwrap(),
            ElementId {
                namespace: vec![],
                name: "ContainerPort".to_string(),
                raw: "docker:index/ContainerPort:ContainerPort".to_string(),
            }
        );
    }

    #[test]
    fn should_handle_without_namespace() {
        let id = "mypkg::BastionShareableLink";
        assert_eq!(
            ElementId::new(id).unwrap(),
            ElementId {
                namespace: vec![],
                name: "BastionShareableLink".to_string(),
                raw: id.to_string(),
            }
        );
    }

    #[test]
    fn should_handle_deeply_nested() {
        let id = "foo-bar:deeply/nested/module:Resource";
        assert_eq!(
            ElementId::new(id).unwrap(),
            ElementId {
                namespace: vec![
                    "deeply".to_string(),
                    "nested".to_string(),
                    "module".to_string()
                ],
                name: "Resource".to_string(),
                raw: id.to_string(),
            }
        )
    }
}
