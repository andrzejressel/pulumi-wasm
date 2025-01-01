#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct AssignmentDynamicScopeFilterTag {
    /// Specifies the tag to filter by.
    #[builder(into)]
    #[serde(rename = "tag")]
    pub r#tag: Box<String>,
    /// Specifies a list of values the defined tag can have.
    #[builder(into)]
    #[serde(rename = "values")]
    pub r#values: Box<Vec<String>>,
}
