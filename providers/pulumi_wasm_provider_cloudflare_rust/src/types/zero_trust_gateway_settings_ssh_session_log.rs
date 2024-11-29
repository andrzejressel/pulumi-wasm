#[derive(serde::Deserialize, serde::Serialize, bon::Builder, Debug)]
#[builder(finish_fn = build_struct)]
pub struct ZeroTrustGatewaySettingsSshSessionLog {
    /// Public key used to encrypt ssh session.
    #[builder(into)]
    #[serde(rename = "publicKey")]
    pub r#public_key: Box<String>,
}
