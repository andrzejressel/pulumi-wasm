use anyhow::{Context, Result};
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
}

#[derive(Debug, PartialEq, Hash, Ord, PartialOrd, Eq)]
pub(crate) struct InputProperty {
    pub(crate) name: String,
    pub(crate) r#type: Type,
}

#[derive(Debug, PartialEq, Hash, Ord, PartialOrd, Eq)]
pub(crate) struct OutputProperty {
    pub(crate) name: String,
    pub(crate) r#type: Type,
}

#[derive(Debug, PartialEq, Hash, Ord, PartialOrd, Eq)]
pub(crate) struct GlobalTypeProperty {
    pub(crate) name: String,
    pub(crate) r#type: Type,
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
