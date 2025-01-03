#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetClusterAddonsConfigRayOperatorConfig {
    #[builder(into)]
    #[serde(rename = "enabled")]
    pub r#enabled: Box<bool>,
    /// The status of Ray Logging, which scrapes Ray cluster logs to Cloud Logging. Defaults to disabled; set enabled = true to enable.
    #[builder(into)]
    #[serde(rename = "rayClusterLoggingConfigs")]
    pub r#ray_cluster_logging_configs: Box<Vec<super::super::types::container::GetClusterAddonsConfigRayOperatorConfigRayClusterLoggingConfig>>,
    /// The status of Ray Cluster monitoring, which shows Ray cluster metrics in Cloud Console. Defaults to disabled; set enabled = true to enable.
    #[builder(into)]
    #[serde(rename = "rayClusterMonitoringConfigs")]
    pub r#ray_cluster_monitoring_configs: Box<Vec<super::super::types::container::GetClusterAddonsConfigRayOperatorConfigRayClusterMonitoringConfig>>,
}
