#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ConfigurationPolicyConfigurationPolicySecurityControlsConfigurationSecurityControlCustomParameter {
    /// An object that specifies parameter values for a control in a configuration policy. See below.
    #[builder(into)]
    #[serde(rename = "parameters")]
    pub r#parameters: Box<Vec<super::super::types::securityhub::ConfigurationPolicyConfigurationPolicySecurityControlsConfigurationSecurityControlCustomParameterParameter>>,
    /// The ID of the security control. For more information see the [Security Hub controls reference] documentation.
    #[builder(into)]
    #[serde(rename = "securityControlId")]
    pub r#security_control_id: Box<String>,
}
