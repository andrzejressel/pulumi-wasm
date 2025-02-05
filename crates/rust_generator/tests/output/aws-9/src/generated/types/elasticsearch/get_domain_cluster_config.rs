#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetDomainClusterConfig {
    /// Configuration block containing cold storage configuration.
    #[builder(into)]
    #[serde(rename = "coldStorageOptions")]
    pub r#cold_storage_options: Box<Vec<super::super::types::elasticsearch::GetDomainClusterConfigColdStorageOption>>,
    /// Number of dedicated master nodes in the cluster.
    #[builder(into)]
    #[serde(rename = "dedicatedMasterCount")]
    pub r#dedicated_master_count: Box<i32>,
    /// Indicates whether dedicated master nodes are enabled for the cluster.
    #[builder(into)]
    #[serde(rename = "dedicatedMasterEnabled")]
    pub r#dedicated_master_enabled: Box<bool>,
    /// Instance type of the dedicated master nodes in the cluster.
    #[builder(into)]
    #[serde(rename = "dedicatedMasterType")]
    pub r#dedicated_master_type: Box<String>,
    /// Number of instances in the cluster.
    #[builder(into)]
    #[serde(rename = "instanceCount")]
    pub r#instance_count: Box<i32>,
    /// Instance type of data nodes in the cluster.
    #[builder(into)]
    #[serde(rename = "instanceType")]
    pub r#instance_type: Box<String>,
    /// The number of warm nodes in the cluster.
    #[builder(into)]
    #[serde(rename = "warmCount")]
    pub r#warm_count: Box<i32>,
    /// Warm storage is enabled.
    #[builder(into)]
    #[serde(rename = "warmEnabled")]
    pub r#warm_enabled: Box<bool>,
    /// The instance type for the Elasticsearch cluster's warm nodes.
    #[builder(into)]
    #[serde(rename = "warmType")]
    pub r#warm_type: Box<String>,
    /// Configuration block containing zone awareness settings.
    #[builder(into)]
    #[serde(rename = "zoneAwarenessConfigs")]
    pub r#zone_awareness_configs: Box<Vec<super::super::types::elasticsearch::GetDomainClusterConfigZoneAwarenessConfig>>,
    /// Indicates whether zone awareness is enabled.
    #[builder(into)]
    #[serde(rename = "zoneAwarenessEnabled")]
    pub r#zone_awareness_enabled: Box<bool>,
}
