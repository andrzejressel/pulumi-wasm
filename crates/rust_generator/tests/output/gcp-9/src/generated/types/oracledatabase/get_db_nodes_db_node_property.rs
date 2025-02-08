#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct GetDbNodesDbNodeProperty {
    /// Output only
    #[builder(into)]
    #[serde(rename = "dbNodeStorageSizeGb")]
    pub r#db_node_storage_size_gb: Box<i32>,
    /// Output only
    #[builder(into)]
    #[serde(rename = "dbServerOcid")]
    pub r#db_server_ocid: Box<String>,
    /// Output only
    #[builder(into)]
    #[serde(rename = "hostname")]
    pub r#hostname: Box<String>,
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
    #[serde(rename = "totalCpuCoreCount")]
    pub r#total_cpu_core_count: Box<i32>,
}
