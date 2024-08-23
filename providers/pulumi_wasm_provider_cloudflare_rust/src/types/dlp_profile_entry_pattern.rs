#[derive(serde::Serialize)]
pub struct DlpProfileEntryPattern {
    #[serde(rename = "regex")]
    pub r#regex: Box<String>,
    #[serde(rename = "validation")]
    pub r#validation: Box<Option<String>>,
}
