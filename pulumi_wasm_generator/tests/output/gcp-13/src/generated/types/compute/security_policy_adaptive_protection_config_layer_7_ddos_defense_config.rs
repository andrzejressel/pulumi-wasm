#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct SecurityPolicyAdaptiveProtectionConfigLayer7DdosDefenseConfig {
    /// If set to true, enables CAAP for L7 DDoS detection.
    #[builder(into, default)]
    #[serde(rename = "enable")]
    pub r#enable: Box<Option<bool>>,
    /// Rule visibility. Supported values include: "STANDARD", "PREMIUM".
    #[builder(into, default)]
    #[serde(rename = "ruleVisibility")]
    pub r#rule_visibility: Box<Option<String>>,
    /// Configuration options for layer7 adaptive protection for various customizable thresholds.
    #[builder(into, default)]
    #[serde(rename = "thresholdConfigs")]
    pub r#threshold_configs: Box<Option<Vec<super::super::types::compute::SecurityPolicyAdaptiveProtectionConfigLayer7DdosDefenseConfigThresholdConfig>>>,
}
