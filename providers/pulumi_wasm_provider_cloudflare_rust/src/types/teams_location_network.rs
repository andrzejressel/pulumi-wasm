#[derive(serde::Serialize)]
pub struct TeamsLocationNetwork {
    #[serde(rename = "id")]
    pub r#id: Box<Option<String>>,
    #[serde(rename = "network")]
    pub r#network: Box<String>,
}
