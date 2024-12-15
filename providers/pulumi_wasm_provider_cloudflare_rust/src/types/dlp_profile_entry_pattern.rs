#[derive(serde::Deserialize, serde::Serialize, bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
pub struct DlpProfileEntryPattern {
    /// The regex that defines the pattern.
    #[builder(into)]
    #[serde(rename = "regex")]
    pub r#regex: Box<String>,
    /// The validation algorithm to apply with this pattern.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "validation")]
    pub r#validation: Box<Option<String>>,
}
