#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct RiskConfigurationAccountTakeoverRiskConfiguration {
    /// Account takeover risk configuration actions. See details below.
    #[builder(into)]
    #[serde(rename = "actions")]
    pub r#actions: Box<super::super::types::cognito::RiskConfigurationAccountTakeoverRiskConfigurationActions>,
    /// The notify configuration used to construct email notifications. See details below.
    #[builder(into)]
    #[serde(rename = "notifyConfiguration")]
    pub r#notify_configuration: Box<super::super::types::cognito::RiskConfigurationAccountTakeoverRiskConfigurationNotifyConfiguration>,
}
