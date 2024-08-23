#[derive(serde::Serialize)]
pub struct DlpProfileContextAwarenessSkip {
    #[serde(rename = "files")]
    pub r#files: Box<bool>,
}
