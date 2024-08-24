#[derive(serde::Serialize)]
pub struct SpectrumApplicationOriginDns {
    /// Fully qualified domain name of the origin.
    #[serde(rename = "name")]
    pub r#name: Box<String>,
}
