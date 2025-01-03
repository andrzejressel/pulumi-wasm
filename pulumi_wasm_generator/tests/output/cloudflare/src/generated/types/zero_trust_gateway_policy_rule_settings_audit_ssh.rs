#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ZeroTrustGatewayPolicyRuleSettingsAuditSsh {
    /// Log all SSH commands.
    #[builder(into)]
    #[serde(rename = "commandLogging")]
    pub r#command_logging: Box<bool>,
}
