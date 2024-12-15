#[derive(serde::Deserialize, serde::Serialize, bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
pub struct UnionCase1 {
    #[builder(into)]
    #[serde(rename = "field1")]
    pub r#field_1: Box<String>,
}
