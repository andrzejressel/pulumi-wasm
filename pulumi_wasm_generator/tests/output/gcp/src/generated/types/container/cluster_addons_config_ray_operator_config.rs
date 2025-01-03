#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ClusterAddonsConfigRayOperatorConfig {
    #[builder(into)]
    #[serde(rename = "enabled")]
    pub r#enabled: Box<bool>,
    /// The status of Ray Logging, which scrapes Ray cluster logs to Cloud Logging. Defaults to disabled; set enabled = true to enable.
    #[builder(into, default)]
    #[serde(rename = "rayClusterLoggingConfig")]
    pub r#ray_cluster_logging_config: Box<Option<super::super::types::container::ClusterAddonsConfigRayOperatorConfigRayClusterLoggingConfig>>,
    /// The status of Ray Cluster monitoring, which shows Ray cluster metrics in Cloud Console. Defaults to disabled; set enabled = true to enable.
    #[builder(into, default)]
    #[serde(rename = "rayClusterMonitoringConfig")]
    pub r#ray_cluster_monitoring_config: Box<Option<super::super::types::container::ClusterAddonsConfigRayOperatorConfigRayClusterMonitoringConfig>>,
}
