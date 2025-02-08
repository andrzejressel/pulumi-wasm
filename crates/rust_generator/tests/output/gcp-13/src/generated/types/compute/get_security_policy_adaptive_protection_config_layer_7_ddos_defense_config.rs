#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetSecurityPolicyAdaptiveProtectionConfigLayer7DdosDefenseConfig {
    /// If set to true, enables CAAP for L7 DDoS detection.
    #[builder(into)]
    #[serde(rename = "enable")]
    pub r#enable: Box<bool>,
    /// Rule visibility. Supported values include: "STANDARD", "PREMIUM".
    #[builder(into)]
    #[serde(rename = "ruleVisibility")]
    pub r#rule_visibility: Box<String>,
    /// Configuration options for layer7 adaptive protection for various customizable thresholds.
    #[builder(into)]
    #[serde(rename = "thresholdConfigs")]
    pub r#threshold_configs: Box<Vec<super::super::types::compute::GetSecurityPolicyAdaptiveProtectionConfigLayer7DdosDefenseConfigThresholdConfig>>,
}
