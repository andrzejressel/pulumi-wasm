#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct PartitionIndexPartitionIndex {
    /// Name of the partition index.
    #[builder(into, default)]
    #[serde(rename = "indexName")]
    pub r#index_name: Box<Option<String>>,
    #[builder(into, default)]
    #[serde(rename = "indexStatus")]
    pub r#index_status: Box<Option<String>>,
    /// Keys for the partition index.
    #[builder(into, default)]
    #[serde(rename = "keys")]
    pub r#keys: Box<Option<Vec<String>>>,
}