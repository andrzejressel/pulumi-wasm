#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ListenerRuleActionFixedResponse {
    /// The HTTP response code.
    #[builder(into)]
    #[serde(rename = "statusCode")]
    pub r#status_code: Box<i32>,
}
