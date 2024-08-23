#[derive(serde::Serialize)]
pub struct TeamsAccountPayloadLog {
    #[serde(rename = "publicKey")]
    pub r#public_key: Box<String>,
}
