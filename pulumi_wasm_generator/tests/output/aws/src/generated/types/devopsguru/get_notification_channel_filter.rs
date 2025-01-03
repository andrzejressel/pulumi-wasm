#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetNotificationChannelFilter {
    /// Events to receive notifications for.
    #[builder(into)]
    #[serde(rename = "messageTypes")]
    pub r#message_types: Box<Vec<String>>,
    /// Severity levels to receive notifications for.
    #[builder(into)]
    #[serde(rename = "severities")]
    pub r#severities: Box<Vec<String>>,
}
