#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct CxTestCaseLastTestResultConversationTurnVirtualAgentOutput {
    /// The [Page](https://cloud.google.com/dialogflow/cx/docs/reference/rest/v3/projects.locations.agents.flows.pages#Page) on which the utterance was spoken.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "currentPage")]
    pub r#current_page: Box<Option<super::super::types::diagflow::CxTestCaseLastTestResultConversationTurnVirtualAgentOutputCurrentPage>>,
    /// The list of differences between the original run and the replay for this output, if any.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "differences")]
    pub r#differences: Box<Option<Vec<super::super::types::diagflow::CxTestCaseLastTestResultConversationTurnVirtualAgentOutputDifference>>>,
    /// The session parameters available to the bot at this point.
    #[builder(into, default)]
    #[serde(rename = "sessionParameters")]
    pub r#session_parameters: Box<Option<String>>,
    /// Response error from the agent in the test result. If set, other output is empty.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "status")]
    pub r#status: Box<Option<super::super::types::diagflow::CxTestCaseLastTestResultConversationTurnVirtualAgentOutputStatus>>,
    /// The text responses from the agent for the turn.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "textResponses")]
    pub r#text_responses: Box<Option<Vec<super::super::types::diagflow::CxTestCaseLastTestResultConversationTurnVirtualAgentOutputTextResponse>>>,
    /// The [Intent](https://cloud.google.com/dialogflow/cx/docs/reference/rest/v3/projects.locations.agents.intents#Intent) that triggered the response.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "triggeredIntent")]
    pub r#triggered_intent: Box<Option<super::super::types::diagflow::CxTestCaseLastTestResultConversationTurnVirtualAgentOutputTriggeredIntent>>,
}
