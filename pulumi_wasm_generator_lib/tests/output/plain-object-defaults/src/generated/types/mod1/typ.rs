#[derive(serde::Deserialize, serde::Serialize, bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
pub struct Typ {
    #[builder(into, default)]
    #[serde(rename = "val")]
    pub r#val: Box<Option<String>>,
}