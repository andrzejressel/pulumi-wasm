#[derive(serde::Deserialize, serde::Serialize, bon::Builder, Debug)]
#[builder(finish_fn = build_struct)]
pub struct ZeroTrustGatewayPolicyRuleSettingsAuditSsh {
    /// Log all SSH commands.
    #[builder(into)]
    #[serde(rename = "commandLogging")]
    pub r#command_logging: Box<bool>,
}
