#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct TeamsAccountPayloadLog {
    /// Public key used to encrypt matched payloads.
    #[serde(rename = "publicKey")]
    pub r#public_key: Box<String>,
}
