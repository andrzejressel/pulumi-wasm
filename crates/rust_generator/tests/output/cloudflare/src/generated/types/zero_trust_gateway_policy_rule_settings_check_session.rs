#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ZeroTrustGatewayPolicyRuleSettingsCheckSession {
    /// Configure how fresh the session needs to be to be considered valid.
    #[builder(into)]
    #[serde(rename = "duration")]
    pub r#duration: Box<String>,
    /// Enable session enforcement for this rule.
    #[builder(into)]
    #[serde(rename = "enforce")]
    pub r#enforce: Box<bool>,
}
