#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct V2ModelsIntentConfirmationSettingPromptSpecificationPromptAttemptsSpecificationAudioAndDtmfInputSpecificationAudioSpecification {
    /// Time for which a bot waits after the customer stops speaking to assume the utterance is finished.
    #[builder(into)]
    #[serde(rename = "endTimeoutMs")]
    pub r#end_timeout_ms: Box<i32>,
    /// Time for how long Amazon Lex waits before speech input is truncated and the speech is returned to application.
    #[builder(into)]
    #[serde(rename = "maxLengthMs")]
    pub r#max_length_ms: Box<i32>,
}
