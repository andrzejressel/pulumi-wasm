#[derive(serde::Serialize)]
pub struct DlpProfileEntry {
    #[serde(rename = "enabled")]
    pub r#enabled: Box<Option<bool>>,
    #[serde(rename = "id")]
    pub r#id: Box<Option<String>>,
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    #[serde(rename = "pattern")]
    pub r#pattern: Box<Option<crate::types::DlpProfileEntryPattern>>,
}
