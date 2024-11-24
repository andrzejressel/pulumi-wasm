#[derive(serde::Deserialize, serde::Serialize, bon::Builder, Debug)]
#[builder(finish_fn = build_struct)]
pub struct ZeroTrustGatewayPolicyRuleSettingsUntrustedCert {
    /// Action to be taken when the SSL certificate of upstream is invalid. Available values: `pass_through`, `block`, `error`.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "action")]
    pub r#action: Box<Option<String>>,
}
