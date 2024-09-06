#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct DlpProfileEntryPattern {
    /// The regex that defines the pattern.
    #[serde(rename = "regex")]
    pub r#regex: Box<String>,
    /// The validation algorithm to apply with this pattern.
    #[serde(rename = "validation")]
    pub r#validation: Box<Option<String>>,
}
