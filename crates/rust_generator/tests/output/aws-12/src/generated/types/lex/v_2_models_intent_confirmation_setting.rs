#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct V2ModelsIntentConfirmationSetting {
    /// Whether the intent's confirmation is sent to the user. When this field is false, confirmation and declination responses aren't sent. If the active field isn't specified, the default is true.
    #[builder(into, default)]
    #[serde(rename = "active")]
    pub r#active: Box<Option<bool>>,
    /// Configuration block for the intent's confirmation step. The dialog code hook is triggered based on these invocation settings when the confirmation next step or declination next step or failure next step is `invoke_dialog_code_hook`.  See `code_hook`.
    #[builder(into, default)]
    #[serde(rename = "codeHook")]
    pub r#code_hook: Box<Option<super::super::types::lex::V2ModelsIntentConfirmationSettingCodeHook>>,
    /// Configuration block for conditional branches to evaluate after the intent is closed. See `confirmation_conditional`.
    #[builder(into, default)]
    #[serde(rename = "confirmationConditional")]
    pub r#confirmation_conditional: Box<Option<super::super::types::lex::V2ModelsIntentConfirmationSettingConfirmationConditional>>,
    /// Configuration block for the next step that the bot executes when the customer confirms the intent. See `confirmation_next_step`.
    #[builder(into, default)]
    #[serde(rename = "confirmationNextStep")]
    pub r#confirmation_next_step: Box<Option<super::super::types::lex::V2ModelsIntentConfirmationSettingConfirmationNextStep>>,
    /// Configuration block for message groups that Amazon Lex uses to respond the user input. See `confirmation_response`.
    #[builder(into, default)]
    #[serde(rename = "confirmationResponse")]
    pub r#confirmation_response: Box<Option<super::super::types::lex::V2ModelsIntentConfirmationSettingConfirmationResponse>>,
    /// Configuration block for conditional branches to evaluate after the intent is declined. See `declination_conditional`.
    #[builder(into, default)]
    #[serde(rename = "declinationConditional")]
    pub r#declination_conditional: Box<Option<super::super::types::lex::V2ModelsIntentConfirmationSettingDeclinationConditional>>,
    /// Configuration block for the next step that the bot executes when the customer declines the intent. See `declination_next_step`.
    #[builder(into, default)]
    #[serde(rename = "declinationNextStep")]
    pub r#declination_next_step: Box<Option<super::super::types::lex::V2ModelsIntentConfirmationSettingDeclinationNextStep>>,
    /// Configuration block for when the user answers "no" to the question defined in `prompt_specification`, Amazon Lex responds with this response to acknowledge that the intent was canceled. See `declination_response`.
    #[builder(into, default)]
    #[serde(rename = "declinationResponse")]
    pub r#declination_response: Box<Option<super::super::types::lex::V2ModelsIntentConfirmationSettingDeclinationResponse>>,
    /// Configuration block for when the code hook is invoked during confirmation prompt retries. See `elicitation_code_hook`.
    #[builder(into, default)]
    #[serde(rename = "elicitationCodeHook")]
    pub r#elicitation_code_hook: Box<Option<super::super::types::lex::V2ModelsIntentConfirmationSettingElicitationCodeHook>>,
    /// Configuration block for conditional branches. Branches are evaluated in the order that they are entered in the list. The first branch with a condition that evaluates to true is executed. The last branch in the list is the default branch. The default branch should not have any condition expression. The default branch is executed if no other branch has a matching condition. See `failure_conditional`.
    #[builder(into, default)]
    #[serde(rename = "failureConditional")]
    pub r#failure_conditional: Box<Option<super::super::types::lex::V2ModelsIntentConfirmationSettingFailureConditional>>,
    /// Configuration block for the next step to take in the conversation if the confirmation step fails. See `failure_next_step`.
    #[builder(into, default)]
    #[serde(rename = "failureNextStep")]
    pub r#failure_next_step: Box<Option<super::super::types::lex::V2ModelsIntentConfirmationSettingFailureNextStep>>,
    /// Configuration block for message groups that Amazon Lex uses to respond the user input. See `failure_response`.
    #[builder(into, default)]
    #[serde(rename = "failureResponse")]
    pub r#failure_response: Box<Option<super::super::types::lex::V2ModelsIntentConfirmationSettingFailureResponse>>,
    /// Configuration block for prompting the user to confirm the intent. This question should have a yes or no answer. Amazon Lex uses this prompt to ensure that the user acknowledges that the intent is ready for fulfillment. See `prompt_specification`.
    #[builder(into)]
    #[serde(rename = "promptSpecification")]
    pub r#prompt_specification: Box<super::super::types::lex::V2ModelsIntentConfirmationSettingPromptSpecification>,
}
