#[derive(serde::Deserialize, serde::Serialize, bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
pub struct TeamsAccountPayloadLog {
    /// Public key used to encrypt matched payloads.
    #[builder(into)]
    #[serde(rename = "publicKey")]
    pub r#public_key: Box<String>,
}
