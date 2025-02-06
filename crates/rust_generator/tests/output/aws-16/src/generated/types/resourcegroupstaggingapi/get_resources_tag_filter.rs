#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetResourcesTagFilter {
    /// One part of a key-value pair that makes up a tag.
    #[builder(into)]
    #[serde(rename = "key")]
    pub r#key: Box<String>,
    /// Optional part of a key-value pair that make up a tag.
    #[builder(into, default)]
    #[serde(rename = "values")]
    pub r#values: Box<Option<Vec<String>>>,
}
