#[derive(serde::Deserialize, serde::Serialize, bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
pub struct DlpProfileContextAwareness {
    /// Scan the context of predefined entries to only return matches surrounded by keywords.
    #[builder(into)]
    #[serde(rename = "enabled")]
    pub r#enabled: Box<bool>,
    /// Content types to exclude from context analysis and return all matches.
    #[builder(into)]
    #[serde(rename = "skip")]
    pub r#skip: Box<super::types::DlpProfileContextAwarenessSkip>,
}
