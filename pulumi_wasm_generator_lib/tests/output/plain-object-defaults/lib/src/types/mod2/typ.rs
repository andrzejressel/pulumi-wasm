//! A test for namespaces (mod 2)

#[derive(serde::Deserialize, serde::Serialize, bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
pub struct Typ {
    #[builder(into, default)]
    #[serde(rename = "mod1")]
    pub r#mod_1: Box<Option<crate::types::mod1::Typ>>,
    #[builder(into, default)]
    #[serde(rename = "val")]
    pub r#val: Box<Option<String>>,
}
