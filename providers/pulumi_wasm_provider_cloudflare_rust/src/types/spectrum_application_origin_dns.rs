#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct SpectrumApplicationOriginDns {
    /// Fully qualified domain name of the origin.
    #[serde(rename = "name")]
    pub r#name: Box<String>,
}
