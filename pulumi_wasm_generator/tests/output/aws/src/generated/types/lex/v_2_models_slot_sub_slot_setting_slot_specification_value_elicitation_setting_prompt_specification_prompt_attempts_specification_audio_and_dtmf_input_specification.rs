#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct V2ModelsSlotSubSlotSettingSlotSpecificationValueElicitationSettingPromptSpecificationPromptAttemptsSpecificationAudioAndDtmfInputSpecification {
    #[builder(into, default)]
    #[serde(rename = "audioSpecification")]
    pub r#audio_specification: Box<Option<super::super::types::lex::V2ModelsSlotSubSlotSettingSlotSpecificationValueElicitationSettingPromptSpecificationPromptAttemptsSpecificationAudioAndDtmfInputSpecificationAudioSpecification>>,
    #[builder(into, default)]
    #[serde(rename = "dtmfSpecification")]
    pub r#dtmf_specification: Box<Option<super::super::types::lex::V2ModelsSlotSubSlotSettingSlotSpecificationValueElicitationSettingPromptSpecificationPromptAttemptsSpecificationAudioAndDtmfInputSpecificationDtmfSpecification>>,
    #[builder(into)]
    #[serde(rename = "startTimeoutMs")]
    pub r#start_timeout_ms: Box<i32>,
}
