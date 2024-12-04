use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub(crate) struct YamlFile {
    #[serde(default)]
    pub(crate) resources: BTreeMap<String, YamlResource>,
    #[serde(default)]
    pub(crate) variables: BTreeMap<String, YamlVariable>
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
pub(crate) struct YamlVariable {
    #[serde(rename = "fn::invoke")]
    pub fn_invoke: YamlFnInvoke,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub(crate) struct YamlFnInvoke {
    #[serde(rename = "Function")]
    pub(crate) function: String,
    #[serde(rename = "Arguments")]
    pub(crate) arguments: BTreeMap<String, YamlExpression>,
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
