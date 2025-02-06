#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct V2ModelsSlotValueElicitationSettingPromptSpecification {
    #[builder(into, default)]
    #[serde(rename = "allowInterrupt")]
    pub r#allow_interrupt: Box<Option<bool>>,
    #[builder(into)]
    #[serde(rename = "maxRetries")]
    pub r#max_retries: Box<i32>,
    #[builder(into, default)]
    #[serde(rename = "messageGroups")]
    pub r#message_groups: Box<Option<Vec<super::super::types::lex::V2ModelsSlotValueElicitationSettingPromptSpecificationMessageGroup>>>,
    #[builder(into, default)]
    #[serde(rename = "messageSelectionStrategy")]
    pub r#message_selection_strategy: Box<Option<String>>,
    #[builder(into, default)]
    #[serde(rename = "promptAttemptsSpecifications")]
    pub r#prompt_attempts_specifications: Box<Option<Vec<super::super::types::lex::V2ModelsSlotValueElicitationSettingPromptSpecificationPromptAttemptsSpecification>>>,
}
