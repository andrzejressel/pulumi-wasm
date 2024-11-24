#[derive(serde::Deserialize, serde::Serialize, bon::Builder, Debug)]
#[builder(finish_fn = build_struct)]
pub struct ZeroTrustGatewaySettingsCertificate {
    /// ID of certificate for TLS interception.
    #[builder(into)]
    #[serde(rename = "id")]
    pub r#id: Box<String>,
}
