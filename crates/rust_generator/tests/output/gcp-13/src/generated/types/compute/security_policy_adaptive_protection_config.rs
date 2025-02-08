#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct SecurityPolicyAdaptiveProtectionConfig {
    /// Configuration for [Automatically deploy Adaptive Protection suggested rules](https://cloud.google.com/armor/docs/adaptive-protection-auto-deploy?hl=en). Structure is documented below.
    /// 
    /// <a name="nested_layer_7_ddos_defense_config"></a>The `layer_7_ddos_defense_config` block supports:
    #[builder(into, default)]
    #[serde(rename = "autoDeployConfig")]
    pub r#auto_deploy_config: Box<Option<super::super::types::compute::SecurityPolicyAdaptiveProtectionConfigAutoDeployConfig>>,
    /// Configuration for [Google Cloud Armor Adaptive Protection Layer 7 DDoS Defense](https://cloud.google.com/armor/docs/adaptive-protection-overview?hl=en). Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "layer7DdosDefenseConfig")]
    pub r#layer_7_ddos_defense_config: Box<Option<super::super::types::compute::SecurityPolicyAdaptiveProtectionConfigLayer7DdosDefenseConfig>>,
}
