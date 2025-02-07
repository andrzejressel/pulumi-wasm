#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct CxTestCaseLastTestResultConversationTurnVirtualAgentOutputStatus {
    /// The status code, which should be an enum value of google.rpc.Code.
    #[builder(into, default)]
    #[serde(rename = "code")]
    pub r#code: Box<Option<i32>>,
    /// A JSON encoded list of messages that carry the error details.
    #[builder(into, default)]
    #[serde(rename = "details")]
    pub r#details: Box<Option<String>>,
    /// A developer-facing error message.
    #[builder(into, default)]
    #[serde(rename = "message")]
    pub r#message: Box<Option<String>>,
}
