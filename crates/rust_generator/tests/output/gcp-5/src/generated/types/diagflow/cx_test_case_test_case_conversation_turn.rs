#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct CxTestCaseTestCaseConversationTurn {
    /// The user input.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "userInput")]
    pub r#user_input: Box<Option<super::super::types::diagflow::CxTestCaseTestCaseConversationTurnUserInput>>,
    /// The virtual agent output.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "virtualAgentOutput")]
    pub r#virtual_agent_output: Box<Option<super::super::types::diagflow::CxTestCaseTestCaseConversationTurnVirtualAgentOutput>>,
}
