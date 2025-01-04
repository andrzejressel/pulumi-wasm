#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct SourceIamBindingCondition {
    /// The description of the source (max of 1024 characters).
    #[builder(into, default)]
    #[serde(rename = "description")]
    pub r#description: Box<Option<String>>,
    #[builder(into)]
    #[serde(rename = "expression")]
    pub r#expression: Box<String>,
    #[builder(into)]
    #[serde(rename = "title")]
    pub r#title: Box<String>,
}
