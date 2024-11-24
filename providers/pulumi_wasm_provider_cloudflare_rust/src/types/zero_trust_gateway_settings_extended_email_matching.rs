#[derive(serde::Deserialize, serde::Serialize, bon::Builder, Debug)]
#[builder(finish_fn = build_struct)]
pub struct ZeroTrustGatewaySettingsExtendedEmailMatching {
    /// Whether e-mails should be matched on all variants of user emails (with + or . modifiers) in Firewall policies.
    #[builder(into)]
    #[serde(rename = "enabled")]
    pub r#enabled: Box<bool>,
}
