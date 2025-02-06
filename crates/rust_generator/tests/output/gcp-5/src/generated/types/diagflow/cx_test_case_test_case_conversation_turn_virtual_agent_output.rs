#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct CxTestCaseTestCaseConversationTurnVirtualAgentOutput {
    /// The [Page](https://cloud.google.com/dialogflow/cx/docs/reference/rest/v3/projects.locations.agents.flows.pages#Page) on which the utterance was spoken.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "currentPage")]
    pub r#current_page: Box<Option<super::super::types::diagflow::CxTestCaseTestCaseConversationTurnVirtualAgentOutputCurrentPage>>,
    /// The session parameters available to the bot at this point.
    #[builder(into, default)]
    #[serde(rename = "sessionParameters")]
    pub r#session_parameters: Box<Option<String>>,
    /// The text responses from the agent for the turn.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "textResponses")]
    pub r#text_responses: Box<Option<Vec<super::super::types::diagflow::CxTestCaseTestCaseConversationTurnVirtualAgentOutputTextResponse>>>,
    /// The [Intent](https://cloud.google.com/dialogflow/cx/docs/reference/rest/v3/projects.locations.agents.intents#Intent) that triggered the response.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "triggeredIntent")]
    pub r#triggered_intent: Box<Option<super::super::types::diagflow::CxTestCaseTestCaseConversationTurnVirtualAgentOutputTriggeredIntent>>,
}
