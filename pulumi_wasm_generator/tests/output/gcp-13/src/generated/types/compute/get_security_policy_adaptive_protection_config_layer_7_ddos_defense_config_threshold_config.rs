#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetSecurityPolicyAdaptiveProtectionConfigLayer7DdosDefenseConfigThresholdConfig {
    #[builder(into)]
    #[serde(rename = "autoDeployConfidenceThreshold")]
    pub r#auto_deploy_confidence_threshold: Box<f64>,
    #[builder(into)]
    #[serde(rename = "autoDeployExpirationSec")]
    pub r#auto_deploy_expiration_sec: Box<i32>,
    #[builder(into)]
    #[serde(rename = "autoDeployImpactedBaselineThreshold")]
    pub r#auto_deploy_impacted_baseline_threshold: Box<f64>,
    #[builder(into)]
    #[serde(rename = "autoDeployLoadThreshold")]
    pub r#auto_deploy_load_threshold: Box<f64>,
    #[builder(into)]
    #[serde(rename = "detectionAbsoluteQps")]
    pub r#detection_absolute_qps: Box<f64>,
    #[builder(into)]
    #[serde(rename = "detectionLoadThreshold")]
    pub r#detection_load_threshold: Box<f64>,
    #[builder(into)]
    #[serde(rename = "detectionRelativeToBaselineQps")]
    pub r#detection_relative_to_baseline_qps: Box<f64>,
    /// The name of the security policy. Provide either this or a `self_link`.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    #[builder(into)]
    #[serde(rename = "trafficGranularityConfigs")]
    pub r#traffic_granularity_configs: Box<Vec<super::super::types::compute::GetSecurityPolicyAdaptiveProtectionConfigLayer7DdosDefenseConfigThresholdConfigTrafficGranularityConfig>>,
}
