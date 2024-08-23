#[derive(serde::Serialize)]
pub struct DlpProfileContextAwareness {
    #[serde(rename = "enabled")]
    pub r#enabled: Box<bool>,
    #[serde(rename = "skip")]
    pub r#skip: Box<crate::types::DlpProfileContextAwarenessSkip>,
}
