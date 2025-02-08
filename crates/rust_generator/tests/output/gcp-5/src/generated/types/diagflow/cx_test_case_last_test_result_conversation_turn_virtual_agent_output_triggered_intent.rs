#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct CxTestCaseLastTestResultConversationTurnVirtualAgentOutputTriggeredIntent {
    /// (Output)
    /// The human-readable name of the intent, unique within the agent.
    #[builder(into, default)]
    #[serde(rename = "displayName")]
    pub r#display_name: Box<Option<String>>,
    /// The unique identifier of the intent.
    /// Format: projects/<Project ID>/locations/<Location ID>/agents/<Agent ID>/intents/<Intent ID>.
    #[builder(into, default)]
    #[serde(rename = "name")]
    pub r#name: Box<Option<String>>,
}
