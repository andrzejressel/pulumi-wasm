#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct IntentFollowUpPrompt {
    /// Prompts for information from the user. Attributes are documented under prompt.
    #[builder(into)]
    #[serde(rename = "prompt")]
    pub r#prompt: Box<super::super::types::lex::IntentFollowUpPromptPrompt>,
    /// If the user answers "no" to the question defined in the prompt field,
    /// Amazon Lex responds with this statement to acknowledge that the intent was canceled. Attributes are
    /// documented below under statement.
    #[builder(into)]
    #[serde(rename = "rejectionStatement")]
    pub r#rejection_statement: Box<super::super::types::lex::IntentFollowUpPromptRejectionStatement>,
}
