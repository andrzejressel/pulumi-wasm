#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct V2ModelsSlotSubSlotSettingSlotSpecificationValueElicitationSettingPromptSpecificationMessageGroupMessage {
    #[builder(into, default)]
    #[serde(rename = "customPayload")]
    pub r#custom_payload: Box<Option<super::super::types::lex::V2ModelsSlotSubSlotSettingSlotSpecificationValueElicitationSettingPromptSpecificationMessageGroupMessageCustomPayload>>,
    #[builder(into, default)]
    #[serde(rename = "imageResponseCard")]
    pub r#image_response_card: Box<Option<super::super::types::lex::V2ModelsSlotSubSlotSettingSlotSpecificationValueElicitationSettingPromptSpecificationMessageGroupMessageImageResponseCard>>,
    #[builder(into, default)]
    #[serde(rename = "plainTextMessage")]
    pub r#plain_text_message: Box<Option<super::super::types::lex::V2ModelsSlotSubSlotSettingSlotSpecificationValueElicitationSettingPromptSpecificationMessageGroupMessagePlainTextMessage>>,
    #[builder(into, default)]
    #[serde(rename = "ssmlMessage")]
    pub r#ssml_message: Box<Option<super::super::types::lex::V2ModelsSlotSubSlotSettingSlotSpecificationValueElicitationSettingPromptSpecificationMessageGroupMessageSsmlMessage>>,
}
