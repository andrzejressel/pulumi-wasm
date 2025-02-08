#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct V2ModelsIntentConfirmationSettingPromptSpecification {
    /// Whether the user can interrupt a speech prompt from the bot.
    #[builder(into, default)]
    #[serde(rename = "allowInterrupt")]
    pub r#allow_interrupt: Box<Option<bool>>,
    /// Maximum number of times the bot tries to elicit a response from the user using this prompt.
    #[builder(into)]
    #[serde(rename = "maxRetries")]
    pub r#max_retries: Box<i32>,
    /// Configuration block for messages that Amazon Lex can send to the user. Amazon Lex chooses the actual message to send at runtime. See `message_group`.
    #[builder(into, default)]
    #[serde(rename = "messageGroups")]
    pub r#message_groups: Box<Option<Vec<super::super::types::lex::V2ModelsIntentConfirmationSettingPromptSpecificationMessageGroup>>>,
    /// How a message is selected from a message group among retries. Valid values are `Random` and `Ordered`.
    #[builder(into, default)]
    #[serde(rename = "messageSelectionStrategy")]
    pub r#message_selection_strategy: Box<Option<String>>,
    /// Configuration block for advanced settings on each attempt of the prompt. See `prompt_attempts_specification`.
    #[builder(into, default)]
    #[serde(rename = "promptAttemptsSpecifications")]
    pub r#prompt_attempts_specifications: Box<Option<Vec<super::super::types::lex::V2ModelsIntentConfirmationSettingPromptSpecificationPromptAttemptsSpecification>>>,
}
