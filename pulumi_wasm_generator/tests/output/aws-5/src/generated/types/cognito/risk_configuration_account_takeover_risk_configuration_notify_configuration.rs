#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct RiskConfigurationAccountTakeoverRiskConfigurationNotifyConfiguration {
    /// Email template used when a detected risk event is blocked. See notify email type below.
    #[builder(into, default)]
    #[serde(rename = "blockEmail")]
    pub r#block_email: Box<Option<super::super::types::cognito::RiskConfigurationAccountTakeoverRiskConfigurationNotifyConfigurationBlockEmail>>,
    /// The email address that is sending the email. The address must be either individually verified with Amazon Simple Email Service, or from a domain that has been verified with Amazon SES.
    #[builder(into, default)]
    #[serde(rename = "from")]
    pub r#from: Box<Option<String>>,
    /// The multi-factor authentication (MFA) email template used when MFA is challenged as part of a detected risk. See notify email type below.
    #[builder(into, default)]
    #[serde(rename = "mfaEmail")]
    pub r#mfa_email: Box<Option<super::super::types::cognito::RiskConfigurationAccountTakeoverRiskConfigurationNotifyConfigurationMfaEmail>>,
    /// The email template used when a detected risk event is allowed. See notify email type below.
    #[builder(into, default)]
    #[serde(rename = "noActionEmail")]
    pub r#no_action_email: Box<Option<super::super::types::cognito::RiskConfigurationAccountTakeoverRiskConfigurationNotifyConfigurationNoActionEmail>>,
    /// The destination to which the receiver of an email should reply to.
    #[builder(into, default)]
    #[serde(rename = "replyTo")]
    pub r#reply_to: Box<Option<String>>,
    /// The Amazon Resource Name (ARN) of the identity that is associated with the sending authorization policy. This identity permits Amazon Cognito to send for the email address specified in the From parameter.
    #[builder(into)]
    #[serde(rename = "sourceArn")]
    pub r#source_arn: Box<String>,
}
