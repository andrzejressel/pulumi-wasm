#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct V2ModelsIntentClosingSetting {
    /// Whether an intent's closing response is used. When this field is false, the closing response isn't sent to the user. If the active field isn't specified, the default is true.
    #[builder(into, default)]
    #[serde(rename = "active")]
    pub r#active: Box<Option<bool>>,
    /// Configuration block for response that Amazon Lex sends to the user when the intent is complete. See `closing_response`.
    #[builder(into, default)]
    #[serde(rename = "closingResponse")]
    pub r#closing_response: Box<Option<super::super::types::lex::V2ModelsIntentClosingSettingClosingResponse>>,
    /// Configuration block for list of conditional branches associated with the intent's closing response. These branches are executed when the `next_step` attribute is set to `EvalutateConditional`. See `conditional`.
    #[builder(into, default)]
    #[serde(rename = "conditional")]
    pub r#conditional: Box<Option<super::super::types::lex::V2ModelsIntentClosingSettingConditional>>,
    /// Next step that the bot executes after playing the intent's closing response. See `next_step`.
    #[builder(into, default)]
    #[serde(rename = "nextStep")]
    pub r#next_step: Box<Option<super::super::types::lex::V2ModelsIntentClosingSettingNextStep>>,
}
