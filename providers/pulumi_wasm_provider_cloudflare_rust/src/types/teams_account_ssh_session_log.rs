#[derive(serde::Serialize)]
pub struct TeamsAccountSshSessionLog {
    #[serde(rename = "publicKey")]
    pub r#public_key: Box<String>,
}
