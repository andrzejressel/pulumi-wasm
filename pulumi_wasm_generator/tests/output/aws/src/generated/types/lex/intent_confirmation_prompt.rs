#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct IntentConfirmationPrompt {
    /// The number of times to prompt the user for information. Must be a number between 1 and 5 (inclusive).
    #[builder(into)]
    #[serde(rename = "maxAttempts")]
    pub r#max_attempts: Box<i32>,
    #[builder(into)]
    #[serde(rename = "messages")]
    pub r#messages: Box<Vec<super::super::types::lex::IntentConfirmationPromptMessage>>,
    #[builder(into, default)]
    #[serde(rename = "responseCard")]
    pub r#response_card: Box<Option<String>>,
}