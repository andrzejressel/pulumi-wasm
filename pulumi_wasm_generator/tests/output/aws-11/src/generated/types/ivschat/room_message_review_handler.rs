#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct RoomMessageReviewHandler {
    /// The fallback behavior (whether the message
    /// is allowed or denied) if the handler does not return a valid response,
    /// encounters an error, or times out. Valid values: `ALLOW`, `DENY`.
    #[builder(into, default)]
    #[serde(rename = "fallbackResult")]
    pub r#fallback_result: Box<Option<String>>,
    /// ARN of the lambda message review handler function.
    #[builder(into, default)]
    #[serde(rename = "uri")]
    pub r#uri: Box<Option<String>>,
}
