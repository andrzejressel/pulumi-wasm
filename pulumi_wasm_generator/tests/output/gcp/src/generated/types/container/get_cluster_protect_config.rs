#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetClusterProtectConfig {
    /// WorkloadConfig defines which actions are enabled for a cluster's workload configurations.
    #[builder(into)]
    #[serde(rename = "workloadConfigs")]
    pub r#workload_configs: Box<Vec<super::super::types::container::GetClusterProtectConfigWorkloadConfig>>,
    /// Sets which mode to use for Protect workload vulnerability scanning feature. Accepted values are DISABLED, BASIC.
    #[builder(into)]
    #[serde(rename = "workloadVulnerabilityMode")]
    pub r#workload_vulnerability_mode: Box<String>,
}
