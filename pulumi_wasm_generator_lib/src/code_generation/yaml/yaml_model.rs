use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub(crate) struct YamlFile {
    pub(crate) resources: BTreeMap<String, YamlResource>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub(crate) struct YamlResource {
    #[serde(rename = "type")]
    pub(crate) type_: String,
    pub(crate) name: Option<String>,
    #[serde(default)]
    pub(crate) properties: BTreeMap<String, YamlExpression>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub(crate) enum YamlExpression {
    String(String),
    Number(f64),
    Boolean(bool),
    Object(BTreeMap<String, YamlExpression>),
    Array(Vec<YamlExpression>),
}

impl YamlFile {
    pub fn from_yaml(yaml: &str) -> Result<Self, serde_yaml::Error> {
        serde_yaml::from_str(yaml)
    }
}
