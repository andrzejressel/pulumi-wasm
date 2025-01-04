#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct V2ModelsSlotValueElicitationSettingWaitAndContinueSpecificationStillWaitingResponseMessageGroupVariation {
    #[builder(into, default)]
    #[serde(rename = "customPayloads")]
    pub r#custom_payloads: Box<Option<Vec<super::super::types::lex::V2ModelsSlotValueElicitationSettingWaitAndContinueSpecificationStillWaitingResponseMessageGroupVariationCustomPayload>>>,
    #[builder(into, default)]
    #[serde(rename = "imageResponseCard")]
    pub r#image_response_card: Box<Option<super::super::types::lex::V2ModelsSlotValueElicitationSettingWaitAndContinueSpecificationStillWaitingResponseMessageGroupVariationImageResponseCard>>,
    #[builder(into, default)]
    #[serde(rename = "plainTextMessage")]
    pub r#plain_text_message: Box<Option<super::super::types::lex::V2ModelsSlotValueElicitationSettingWaitAndContinueSpecificationStillWaitingResponseMessageGroupVariationPlainTextMessage>>,
    #[builder(into, default)]
    #[serde(rename = "ssmlMessage")]
    pub r#ssml_message: Box<Option<super::super::types::lex::V2ModelsSlotValueElicitationSettingWaitAndContinueSpecificationStillWaitingResponseMessageGroupVariationSsmlMessage>>,
}
