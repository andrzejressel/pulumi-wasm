#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetCatalogTablePartitionIndex {
    /// Name of the partition index.
    #[builder(into)]
    #[serde(rename = "indexName")]
    pub r#index_name: Box<String>,
    #[builder(into)]
    #[serde(rename = "indexStatus")]
    pub r#index_status: Box<String>,
    /// Keys for the partition index.
    #[builder(into)]
    #[serde(rename = "keys")]
    pub r#keys: Box<Vec<String>>,
}