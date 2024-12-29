#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct DomainClusterConfig {
    /// Configuration block containing cold storage configuration. Detailed below.
    #[builder(into, default)]
    #[serde(rename = "coldStorageOptions")]
    pub r#cold_storage_options: Box<Option<super::super::types::elasticsearch::DomainClusterConfigColdStorageOptions>>,
    /// Number of dedicated main nodes in the cluster.
    #[builder(into, default)]
    #[serde(rename = "dedicatedMasterCount")]
    pub r#dedicated_master_count: Box<Option<i32>>,
    /// Whether dedicated main nodes are enabled for the cluster.
    #[builder(into, default)]
    #[serde(rename = "dedicatedMasterEnabled")]
    pub r#dedicated_master_enabled: Box<Option<bool>>,
    /// Instance type of the dedicated main nodes in the cluster.
    #[builder(into, default)]
    #[serde(rename = "dedicatedMasterType")]
    pub r#dedicated_master_type: Box<Option<String>>,
    /// Number of instances in the cluster.
    #[builder(into, default)]
    #[serde(rename = "instanceCount")]
    pub r#instance_count: Box<Option<i32>>,
    /// Instance type of data nodes in the cluster.
    #[builder(into, default)]
    #[serde(rename = "instanceType")]
    pub r#instance_type: Box<Option<String>>,
    /// Number of warm nodes in the cluster. Valid values are between `2` and `150`. `warm_count` can be only and must be set when `warm_enabled` is set to `true`.
    #[builder(into, default)]
    #[serde(rename = "warmCount")]
    pub r#warm_count: Box<Option<i32>>,
    /// Whether to enable warm storage.
    #[builder(into, default)]
    #[serde(rename = "warmEnabled")]
    pub r#warm_enabled: Box<Option<bool>>,
    /// Instance type for the Elasticsearch cluster's warm nodes. Valid values are `ultrawarm1.medium.elasticsearch`, `ultrawarm1.large.elasticsearch` and `ultrawarm1.xlarge.elasticsearch`. `warm_type` can be only and must be set when `warm_enabled` is set to `true`.
    #[builder(into, default)]
    #[serde(rename = "warmType")]
    pub r#warm_type: Box<Option<String>>,
    /// Configuration block containing zone awareness settings. Detailed below.
    #[builder(into, default)]
    #[serde(rename = "zoneAwarenessConfig")]
    pub r#zone_awareness_config: Box<Option<super::super::types::elasticsearch::DomainClusterConfigZoneAwarenessConfig>>,
    /// Whether zone awareness is enabled, set to `true` for multi-az deployment. To enable awareness with three Availability Zones, the `availability_zone_count` within the `zone_awareness_config` must be set to `3`.
    #[builder(into, default)]
    #[serde(rename = "zoneAwarenessEnabled")]
    pub r#zone_awareness_enabled: Box<Option<bool>>,
}
