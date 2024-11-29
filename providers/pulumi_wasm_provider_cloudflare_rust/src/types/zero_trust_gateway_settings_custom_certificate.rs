#[derive(serde::Deserialize, serde::Serialize, bon::Builder, Debug)]
#[builder(finish_fn = build_struct)]
pub struct ZeroTrustGatewaySettingsCustomCertificate {
    /// Whether TLS encryption should use a custom certificate.
    #[builder(into)]
    #[serde(rename = "enabled")]
    pub r#enabled: Box<bool>,
    /// ID of custom certificate.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "id")]
    pub r#id: Box<Option<String>>,
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "updatedAt")]
    pub r#updated_at: Box<Option<String>>,
}
