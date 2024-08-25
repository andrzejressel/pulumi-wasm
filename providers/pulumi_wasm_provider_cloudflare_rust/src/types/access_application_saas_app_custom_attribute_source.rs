#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct AccessApplicationSaasAppCustomAttributeSource {
    /// The name of the footer link.
    #[serde(rename = "name")]
    pub r#name: Box<String>,
}
