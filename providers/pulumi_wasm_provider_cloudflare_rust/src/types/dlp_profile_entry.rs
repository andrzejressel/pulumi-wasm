#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct DlpProfileEntry {
    /// Whether the entry is active. Defaults to `false`.
    #[serde(rename = "enabled")]
    pub r#enabled: Box<Option<bool>>,
    /// Unique entry identifier.
    #[serde(rename = "id")]
    pub r#id: Box<Option<String>>,
    /// Name of the entry to deploy.
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    #[serde(rename = "pattern")]
    pub r#pattern: Box<Option<crate::types::DlpProfileEntryPattern>>,
}
