#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct CxTestCaseLastTestResultConversationTurn {
    /// The user input.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "userInput")]
    pub r#user_input: Box<Option<super::super::types::diagflow::CxTestCaseLastTestResultConversationTurnUserInput>>,
    /// The virtual agent output.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "virtualAgentOutput")]
    pub r#virtual_agent_output: Box<Option<super::super::types::diagflow::CxTestCaseLastTestResultConversationTurnVirtualAgentOutput>>,
}
