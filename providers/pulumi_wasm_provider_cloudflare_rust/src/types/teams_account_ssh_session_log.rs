#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct TeamsAccountSshSessionLog {
    /// Public key used to encrypt ssh session.
    #[serde(rename = "publicKey")]
    pub r#public_key: Box<String>,
}
