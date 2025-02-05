#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct V2ModelsIntentConfirmationSettingDeclinationResponseMessageGroupVariation {
    /// Configuration block for a message in a custom format defined by the client application. See `custom_payload`.
    #[builder(into, default)]
    #[serde(rename = "customPayload")]
    pub r#custom_payload: Box<Option<super::super::types::lex::V2ModelsIntentConfirmationSettingDeclinationResponseMessageGroupVariationCustomPayload>>,
    /// Configuration block for a message that defines a response card that the client application can show to the user. See `image_response_card`.
    #[builder(into, default)]
    #[serde(rename = "imageResponseCard")]
    pub r#image_response_card: Box<Option<super::super::types::lex::V2ModelsIntentConfirmationSettingDeclinationResponseMessageGroupVariationImageResponseCard>>,
    /// Configuration block for a message in plain text format. See `plain_text_message`.
    #[builder(into, default)]
    #[serde(rename = "plainTextMessage")]
    pub r#plain_text_message: Box<Option<super::super::types::lex::V2ModelsIntentConfirmationSettingDeclinationResponseMessageGroupVariationPlainTextMessage>>,
    /// Configuration block for a message in Speech Synthesis Markup Language (SSML). See `ssml_message`.
    #[builder(into, default)]
    #[serde(rename = "ssmlMessage")]
    pub r#ssml_message: Box<Option<super::super::types::lex::V2ModelsIntentConfirmationSettingDeclinationResponseMessageGroupVariationSsmlMessage>>,
}
