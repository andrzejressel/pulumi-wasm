#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct CxTestCaseTestCaseConversationTurnUserInputInputText {
    /// The natural language text to be processed. Text length must not exceed 256 characters.
    #[builder(into)]
    #[serde(rename = "text")]
    pub r#text: Box<String>,
}
