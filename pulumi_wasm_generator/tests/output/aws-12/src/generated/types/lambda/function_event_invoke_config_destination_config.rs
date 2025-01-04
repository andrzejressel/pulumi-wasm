#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct FunctionEventInvokeConfigDestinationConfig {
    /// Configuration block with destination configuration for failed asynchronous invocations. See below for details.
    #[builder(into, default)]
    #[serde(rename = "onFailure")]
    pub r#on_failure: Box<Option<super::super::types::lambda::FunctionEventInvokeConfigDestinationConfigOnFailure>>,
    /// Configuration block with destination configuration for successful asynchronous invocations. See below for details.
    #[builder(into, default)]
    #[serde(rename = "onSuccess")]
    pub r#on_success: Box<Option<super::super::types::lambda::FunctionEventInvokeConfigDestinationConfigOnSuccess>>,
}
