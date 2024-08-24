#[derive(serde::Serialize)]
pub struct SpectrumApplicationDns {
    /// The name of the DNS record associated with the application.
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// The type of DNS record associated with the application.
    #[serde(rename = "type")]
    pub r#type: Box<String>,
}
