#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct V2ModelsIntentConfirmationSettingFailureConditionalDefaultBranch {
    /// Configuration block for the next step in the conversation. See `next_step`.
    #[builder(into, default)]
    #[serde(rename = "nextStep")]
    pub r#next_step: Box<Option<super::super::types::lex::V2ModelsIntentConfirmationSettingFailureConditionalDefaultBranchNextStep>>,
    /// Configuration block for a list of message groups that Amazon Lex uses to respond to the user input. See `response`.
    #[builder(into, default)]
    #[serde(rename = "response")]
    pub r#response: Box<Option<super::super::types::lex::V2ModelsIntentConfirmationSettingFailureConditionalDefaultBranchResponse>>,
}