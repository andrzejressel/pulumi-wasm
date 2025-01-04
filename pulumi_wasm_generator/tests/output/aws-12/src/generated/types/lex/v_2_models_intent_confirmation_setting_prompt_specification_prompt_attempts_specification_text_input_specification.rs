#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct V2ModelsIntentConfirmationSettingPromptSpecificationPromptAttemptsSpecificationTextInputSpecification {
    /// Time for which a bot waits before re-prompting a customer for text input.
    #[builder(into)]
    #[serde(rename = "startTimeoutMs")]
    pub r#start_timeout_ms: Box<i32>,
}
