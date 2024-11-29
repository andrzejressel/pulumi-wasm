#[derive(serde::Deserialize, serde::Serialize, bon::Builder, Debug)]
#[builder(finish_fn = build_struct)]
pub struct DlpProfileEntry {
    /// Whether the entry is active. Defaults to `false`.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "enabled")]
    pub r#enabled: Box<Option<bool>>,
    /// Unique entry identifier.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "id")]
    pub r#id: Box<Option<String>>,
    /// Name of the entry to deploy.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "pattern")]
    pub r#pattern: Box<Option<crate::types::DlpProfileEntryPattern>>,
}
