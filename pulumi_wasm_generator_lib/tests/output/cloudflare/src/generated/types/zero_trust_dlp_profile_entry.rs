#[derive(serde::Deserialize, serde::Serialize, bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
pub struct ZeroTrustDlpProfileEntry {
    /// Whether the entry is active. Defaults to `false`.
    #[builder(into, default)]
    #[serde(rename = "enabled")]
    pub r#enabled: Box<Option<bool>>,
    /// Unique entry identifier.
    #[builder(into, default)]
    #[serde(rename = "id")]
    pub r#id: Box<Option<String>>,
    /// Name of the entry to deploy.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    #[builder(into, default)]
    #[serde(rename = "pattern")]
    pub r#pattern: Box<Option<super::types::ZeroTrustDlpProfileEntryPattern>>,
}