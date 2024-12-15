#[derive(serde::Deserialize, serde::Serialize, bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
pub struct UnionCaseWithConst1 {
    #[builder(into, default)]
    #[serde(rename = "field")]
    pub r#field: Box<crate::ConstString_1>,
    #[builder(into)]
    #[serde(rename = "field1")]
    pub r#field_1: Box<String>,
}
