#[derive(serde::Deserialize, serde::Serialize, bon::Builder, Debug)]
#[builder(finish_fn = build_struct)]
pub struct ZeroTrustGatewayPolicyRuleSettingsL4Override {
    /// Override IP to forward traffic to.
    #[builder(into)]
    #[serde(rename = "ip")]
    pub r#ip: Box<String>,
    /// Override Port to forward traffic to.
    #[builder(into)]
    #[serde(rename = "port")]
    pub r#port: Box<i32>,
}
