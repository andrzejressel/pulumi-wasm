#[derive(serde::Deserialize, serde::Serialize, bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
pub struct TeamsAccountCustomCertificate {
    /// Whether TLS encryption should use a custom certificate.
    #[builder(into)]
    #[serde(rename = "enabled")]
    pub r#enabled: Box<bool>,
    /// ID of custom certificate.
    #[builder(into, default)]
    #[serde(rename = "id")]
    pub r#id: Box<Option<String>>,
    #[builder(into, default)]
    #[serde(rename = "updatedAt")]
    pub r#updated_at: Box<Option<String>>,
}
