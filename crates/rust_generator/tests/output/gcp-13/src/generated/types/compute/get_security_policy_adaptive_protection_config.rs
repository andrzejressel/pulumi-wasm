#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetSecurityPolicyAdaptiveProtectionConfig {
    /// Auto Deploy Config of this security policy
    #[builder(into)]
    #[serde(rename = "autoDeployConfigs")]
    pub r#auto_deploy_configs: Box<Vec<super::super::types::compute::GetSecurityPolicyAdaptiveProtectionConfigAutoDeployConfig>>,
    /// Layer 7 DDoS Defense Config of this security policy
    #[builder(into)]
    #[serde(rename = "layer7DdosDefenseConfigs")]
    pub r#layer_7_ddos_defense_configs: Box<Vec<super::super::types::compute::GetSecurityPolicyAdaptiveProtectionConfigLayer7DdosDefenseConfig>>,
}
