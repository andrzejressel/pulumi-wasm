#[derive(serde::Deserialize, serde::Serialize, bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
pub struct Typ {
    #[builder(into, default)]
    #[serde(rename = "mod1")]
    pub r#mod_1: Box<Option<super::types::mod1::Typ>>,
    #[builder(into, default)]
    #[serde(rename = "mod2")]
    pub r#mod_2: Box<Option<super::types::mod2::Typ>>,
    #[builder(into, default)]
    #[serde(rename = "val")]
    pub r#val: Box<Option<String>>,
}