#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct DlpProfileContextAwareness {
    /// Scan the context of predefined entries to only return matches surrounded by keywords.
    #[serde(rename = "enabled")]
    pub r#enabled: Box<bool>,
    /// Content types to exclude from context analysis and return all matches.
    #[serde(rename = "skip")]
    pub r#skip: Box<crate::types::DlpProfileContextAwarenessSkip>,
}
