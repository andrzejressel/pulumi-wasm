#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct CxTestCaseLastTestResultConversationTurnVirtualAgentOutputDifference {
    /// A human readable description of the diff, showing the actual output vs expected output.
    #[builder(into, default)]
    #[serde(rename = "description")]
    pub r#description: Box<Option<String>>,
    /// The type of diff.
    /// * INTENT: The intent.
    /// * PAGE: The page.
    /// * PARAMETERS: The parameters.
    /// * UTTERANCE: The message utterance.
    /// * FLOW: The flow.
    /// Possible values are: `INTENT`, `PAGE`, `PARAMETERS`, `UTTERANCE`, `FLOW`.
    #[builder(into, default)]
    #[serde(rename = "type")]
    pub r#type_: Box<Option<String>>,
}
