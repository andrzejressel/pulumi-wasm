#[derive(serde::Serialize)]
pub struct SpectrumApplicationDns {
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    #[serde(rename = "type")]
    pub r#type: Box<String>,
}
