#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct AppLimits {
    /// The maximum number of messages that the campaign can send daily.
    #[builder(into, default)]
    #[serde(rename = "daily")]
    pub r#daily: Box<Option<i32>>,
    /// The length of time (in seconds) that the campaign can run before it ends and message deliveries stop. This duration begins at the scheduled start time for the campaign. The minimum value is 60.
    #[builder(into, default)]
    #[serde(rename = "maximumDuration")]
    pub r#maximum_duration: Box<Option<i32>>,
    /// The number of messages that the campaign can send per second. The minimum value is 50, and the maximum is 20000.
    #[builder(into, default)]
    #[serde(rename = "messagesPerSecond")]
    pub r#messages_per_second: Box<Option<i32>>,
    /// The maximum total number of messages that the campaign can send.
    #[builder(into, default)]
    #[serde(rename = "total")]
    pub r#total: Box<Option<i32>>,
}
