#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct V2ModelsSlotValueElicitationSettingPromptSpecificationPromptAttemptsSpecificationAudioAndDtmfInputSpecificationAudioSpecification {
    #[builder(into)]
    #[serde(rename = "endTimeoutMs")]
    pub r#end_timeout_ms: Box<i32>,
    #[builder(into)]
    #[serde(rename = "maxLengthMs")]
    pub r#max_length_ms: Box<i32>,
}
