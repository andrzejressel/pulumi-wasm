#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct CxTestCaseLastTestResultConversationTurnVirtualAgentOutputTextResponse {
    /// A collection of text responses.
    #[builder(into, default)]
    #[serde(rename = "texts")]
    pub r#texts: Box<Option<Vec<String>>>,
}
