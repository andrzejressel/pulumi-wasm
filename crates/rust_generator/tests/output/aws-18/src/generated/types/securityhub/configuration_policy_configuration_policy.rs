#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ConfigurationPolicyConfigurationPolicy {
    /// A list that defines which security standards are enabled in the configuration policy. It must be defined if `service_enabled` is set to true.
    #[builder(into, default)]
    #[serde(rename = "enabledStandardArns")]
    pub r#enabled_standard_arns: Box<Option<Vec<String>>>,
    /// Defines which security controls are enabled in the configuration policy and any customizations to parameters affecting them. See below.
    #[builder(into, default)]
    #[serde(rename = "securityControlsConfiguration")]
    pub r#security_controls_configuration: Box<Option<super::super::types::securityhub::ConfigurationPolicyConfigurationPolicySecurityControlsConfiguration>>,
    /// Indicates whether Security Hub is enabled in the policy.
    #[builder(into)]
    #[serde(rename = "serviceEnabled")]
    pub r#service_enabled: Box<bool>,
}
