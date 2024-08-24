#[derive(serde::Serialize)]
pub struct DlpProfileContextAwarenessSkip {
    /// Return all matches, regardless of context analysis result, if the data is a file.
    #[serde(rename = "files")]
    pub r#files: Box<bool>,
}
