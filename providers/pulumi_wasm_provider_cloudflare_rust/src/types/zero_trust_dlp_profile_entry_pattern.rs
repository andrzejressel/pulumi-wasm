#[derive(serde::Deserialize, serde::Serialize, bon::Builder, Debug)]
#[builder(finish_fn = build_struct)]
pub struct ZeroTrustDlpProfileEntryPattern {
    /// The regex that defines the pattern.
    #[builder(into)]
    #[serde(rename = "regex")]
    pub r#regex: Box<String>,
    /// The validation algorithm to apply with this pattern.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "validation")]
    pub r#validation: Box<Option<String>>,
}