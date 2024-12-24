#[derive(serde::Deserialize, serde::Serialize, bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
pub struct TeamsAccountSshSessionLog {
    /// Public key used to encrypt ssh session.
    #[builder(into)]
    #[serde(rename = "publicKey")]
    pub r#public_key: Box<String>,
}
