#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct V2ModelsIntentClosingSettingNextStep {
    /// Configuration block for action that the bot executes at runtime when the conversation reaches this step. See `dialog_action`.
    #[builder(into, default)]
    #[serde(rename = "dialogAction")]
    pub r#dialog_action: Box<Option<super::super::types::lex::V2ModelsIntentClosingSettingNextStepDialogAction>>,
    /// Configuration block for override settings to configure the intent state. See `intent`.
    #[builder(into, default)]
    #[serde(rename = "intent")]
    pub r#intent: Box<Option<super::super::types::lex::V2ModelsIntentClosingSettingNextStepIntent>>,
    /// Map of key/value pairs representing session-specific context information. It contains application information passed between Amazon Lex and a client application.
    #[builder(into, default)]
    #[serde(rename = "sessionAttributes")]
    pub r#session_attributes: Box<Option<std::collections::HashMap<String, String>>>,
}
