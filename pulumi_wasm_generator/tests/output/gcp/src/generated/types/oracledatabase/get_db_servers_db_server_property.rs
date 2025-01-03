#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetDbServersDbServerProperty {
    /// Output only
    #[builder(into)]
    #[serde(rename = "dbNodeIds")]
    pub r#db_node_ids: Box<Vec<String>>,
    /// Output only
    #[builder(into)]
    #[serde(rename = "dbNodeStorageSizeGb")]
    pub r#db_node_storage_size_gb: Box<i32>,
    /// Output only
    #[builder(into)]
    #[serde(rename = "maxDbNodeStorageSizeGb")]
    pub r#max_db_node_storage_size_gb: Box<i32>,
    /// Output only
    #[builder(into)]
    #[serde(rename = "maxMemorySizeGb")]
    pub r#max_memory_size_gb: Box<i32>,
    /// Output only
    #[builder(into)]
    #[serde(rename = "maxOcpuCount")]
    pub r#max_ocpu_count: Box<i32>,
    /// Output only
    #[builder(into)]
    #[serde(rename = "memorySizeGb")]
    pub r#memory_size_gb: Box<i32>,
    /// Output only
    #[builder(into)]
    #[serde(rename = "ocid")]
    pub r#ocid: Box<String>,
    /// Output only
    #[builder(into)]
    #[serde(rename = "ocpuCount")]
    pub r#ocpu_count: Box<i32>,
    /// Output only
    #[builder(into)]
    #[serde(rename = "state")]
    pub r#state: Box<String>,
    /// Output only
    #[builder(into)]
    #[serde(rename = "vmCount")]
    pub r#vm_count: Box<i32>,
}
