#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct SecurityPolicyAdaptiveProtectionConfigLayer7DdosDefenseConfigThresholdConfig {
    /// Confidence threshold above which Adaptive Protection's auto-deploy takes actions.
    #[builder(into, default)]
    #[serde(rename = "autoDeployConfidenceThreshold")]
    pub r#auto_deploy_confidence_threshold: Box<Option<f64>>,
    /// Duration over which Adaptive Protection's auto-deployed actions last.
    #[builder(into, default)]
    #[serde(rename = "autoDeployExpirationSec")]
    pub r#auto_deploy_expiration_sec: Box<Option<i32>>,
    /// Impacted baseline threshold below which Adaptive Protection's auto-deploy takes actions.
    #[builder(into, default)]
    #[serde(rename = "autoDeployImpactedBaselineThreshold")]
    pub r#auto_deploy_impacted_baseline_threshold: Box<Option<f64>>,
    /// Load threshold above which Adaptive Protection automatically deploy threshold based on the backend load threshold and detect a new rule during an alerted attack.
    #[builder(into, default)]
    #[serde(rename = "autoDeployLoadThreshold")]
    pub r#auto_deploy_load_threshold: Box<Option<f64>>,
    /// Detection threshold based on absolute QPS.
    #[builder(into, default)]
    #[serde(rename = "detectionAbsoluteQps")]
    pub r#detection_absolute_qps: Box<Option<f64>>,
    /// Detection threshold based on the backend service's load.
    #[builder(into, default)]
    #[serde(rename = "detectionLoadThreshold")]
    pub r#detection_load_threshold: Box<Option<f64>>,
    /// Detection threshold based on QPS relative to the average of baseline traffic.
    #[builder(into, default)]
    #[serde(rename = "detectionRelativeToBaselineQps")]
    pub r#detection_relative_to_baseline_qps: Box<Option<f64>>,
    /// The name of config. The name must be 1-63 characters long, and comply with RFC1035. The name must be unique within the security policy.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// Configuration options for enabling Adaptive Protection to work on the specified service granularity. Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "trafficGranularityConfigs")]
    pub r#traffic_granularity_configs: Box<Option<Vec<super::super::types::compute::SecurityPolicyAdaptiveProtectionConfigLayer7DdosDefenseConfigThresholdConfigTrafficGranularityConfig>>>,
}
