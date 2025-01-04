#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct CassandraTableSchemaClusterKey {
    /// Name of the cluster key to be created.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// Order of the key. Currently supported values are `Asc` and `Desc`.
    #[builder(into)]
    #[serde(rename = "orderBy")]
    pub r#order_by: Box<String>,
}
