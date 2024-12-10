use crate::utils::{escape_rust_name, escape_wit_identifier, replace_multiple_dashes};
use anyhow::{Context, Result};
use convert_case::Case;
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
    DiscriminatedUnion(Vec<Ref>),
}

impl Type {
    pub(crate) fn get_rust_type(&self, input_output: &str, property_name: String) -> String {
        match self {
            Type::Boolean => "bool".into(),
            Type::Integer => "i32".into(),
            Type::Number => "f64".into(),
            Type::String => "String".into(),
            Type::Array(type_) => {
                format!("Vec<{}>", type_.get_rust_type(input_output, property_name))
            }
            Type::Object(type_) => {
                format!(
                    "std::collections::HashMap<String, {}>",
                    type_.get_rust_type(input_output, property_name)
                )
            }
            Type::Ref(r) => match r {
                Ref::Type(tpe) => format!("crate::types::{}", tpe.get_rust_struct_name()),
                Ref::Archive => "String".to_string(), //FIXME
                Ref::Asset => "String".to_string(),   //FIXME
                Ref::Any => "String".to_string(),     //FIXME
            },
            Type::Option(type_) => format!(
                "Option<{}>",
                type_.get_rust_type(input_output, property_name)
            ),
            // Type::DiscriminatedUnion(_, _) => format!("{}_{}", input_output, property_name).to_case(Case::Pascal),
            Type::DiscriminatedUnion(refs) => format!(
                "pulumi_wasm_provider_common::OneOf{}<{}>",
                refs.len(),
                refs
                    .iter()
                    .map(|(r)| Type::Ref(r.clone())
                        .get_rust_type(input_output, property_name.clone()))
                    .collect::<Vec<_>>()
                    .join(", ")
            ),
        }
    }

    pub(crate) fn get_internal_discriminated_union(&self) -> Option<Vec<Type>> {
        match self {
            Type::Boolean => None,
            Type::Integer => None,
            Type::Number => None,
            Type::String => None,
            Type::Array(t) => t.get_internal_discriminated_union(),
            Type::Object(o) => o.get_internal_discriminated_union(),
            Type::Ref(_) => None,
            Type::Option(o) => o.get_internal_discriminated_union(),
            Type::DiscriminatedUnion(m) => {
                Some(m.iter().map(|(r)| Type::Ref(r.clone())).collect())
            }
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
    pub(crate) fn get_wit_argument_name(&self) -> String {
        escape_wit_identifier(ElementId::create_valid_wit_id(self.name.as_str()).as_str()).into()
    }

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
    pub(crate) fn get_wit_argument_name(&self) -> String {
        escape_wit_identifier(ElementId::create_valid_wit_id(self.name.as_str()).as_str()).into()
    }
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

#[derive(Debug, PartialEq, Hash, Ord, PartialOrd, Eq)]
pub(crate) enum GlobalType {
    Object(Vec<GlobalTypeProperty>),
    String,
    Boolean,
    Number,
    Integer,
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

#[derive(Debug, PartialEq, Hash, Ord, PartialOrd, Eq)]
pub(crate) struct Package {
    pub(crate) name: String,
    pub(crate) display_name: Option<String>,
    pub(crate) version: String,
    pub(crate) resources: BTreeMap<ElementId, Resource>,
    pub(crate) functions: BTreeMap<ElementId, Function>,
    pub(crate) types: BTreeMap<ElementId, GlobalType>,
}

impl Package {
    pub(crate) fn get_wit_name(&self) -> String {
        ElementId::create_valid_wit_id(self.name.as_str())
    }
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

    pub(crate) fn get_rust_function_name(&self) -> String {
        self.name
            .clone()
            .from_case(Case::UpperCamel)
            .to_case(Case::Snake)
    }

    pub(crate) fn get_rust_namespace_name(&self) -> String {
        let mut vec = self.namespace.clone();
        vec.push(self.name.clone());
        Self::create_valid_id(&vec.join("-"))
    }

    pub(crate) fn get_wit_interface_name(&self) -> String {
        let mut vec = self.namespace.clone();
        vec.push(self.name.clone());
        escape_wit_identifier(Self::create_valid_wit_id(&vec.join("-")).as_str()).into()
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
        if raw.contains('/') {
            let parts: Vec<&str> = raw.split('/').collect();
            if parts.len() != 2 {
                return Err(anyhow::anyhow!("Cannot generate element id from [{raw}]"));
            }

            let left = parts[0];
            let right = parts[1];

            let parts = right.split(':').collect::<Vec<&str>>();
            if parts.len() != 2 {
                return Err(anyhow::anyhow!("Cannot generate element id from [{raw}]"));
            }

            let name = parts[1].to_string();

            let parts = left.split(':').collect::<Vec<&str>>();
            if parts.len() != 2 {
                return Err(anyhow::anyhow!("Cannot generate element id from [{raw}]"));
            }

            let namespace = match &parts[1] {
                &"index" => vec![],
                package => vec![package.to_string()],
            };

            Ok(ElementId {
                namespace,
                name,
                raw: raw.to_string(),
            })
        } else {
            let parts: Vec<&str> = raw.split(':').collect();
            if parts.len() != 3 {
                return Err(anyhow::anyhow!("Cannot generate element id from [{raw}]"));
            }

            let _package = parts[0].to_string();
            let namespace = match &parts[1] {
                &"index" => vec![],
                package => vec![package.to_string()],
            };
            let name = parts[2].to_string();
            Ok(ElementId {
                namespace,
                name,
                raw: raw.to_string(),
            })
        }
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
}
