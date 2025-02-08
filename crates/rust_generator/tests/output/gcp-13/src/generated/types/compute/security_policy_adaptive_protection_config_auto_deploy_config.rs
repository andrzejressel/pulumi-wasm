#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct SecurityPolicyAdaptiveProtectionConfigAutoDeployConfig {
    /// Rules are only automatically deployed for alerts on potential attacks with confidence scores greater than this threshold.
    #[builder(into, default)]
    #[serde(rename = "confidenceThreshold")]
    pub r#confidence_threshold: Box<Option<f64>>,
    /// Google Cloud Armor stops applying the action in the automatically deployed rule to an identified attacker after this duration. The rule continues to operate against new requests.
    #[builder(into, default)]
    #[serde(rename = "expirationSec")]
    pub r#expiration_sec: Box<Option<i32>>,
    /// Rules are only automatically deployed when the estimated impact to baseline traffic from the suggested mitigation is below this threshold.
    #[builder(into, default)]
    #[serde(rename = "impactedBaselineThreshold")]
    pub r#impacted_baseline_threshold: Box<Option<f64>>,
    /// Identifies new attackers only when the load to the backend service that is under attack exceeds this threshold.
    #[builder(into, default)]
    #[serde(rename = "loadThreshold")]
    pub r#load_threshold: Box<Option<f64>>,
}
