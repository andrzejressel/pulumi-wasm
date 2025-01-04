#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct RuleSourceSourceDetail {
    /// The source of the event, such as an AWS service, that triggers AWS Config to evaluate your AWSresources. This defaults to `aws.config` and is the only valid value.
    #[builder(into, default)]
    #[serde(rename = "eventSource")]
    pub r#event_source: Box<Option<String>>,
    /// The frequency that you want AWS Config to run evaluations for a rule that istriggered periodically. If specified, requires `message_type` to be `ScheduledNotification`.
    #[builder(into, default)]
    #[serde(rename = "maximumExecutionFrequency")]
    pub r#maximum_execution_frequency: Box<Option<String>>,
    /// The type of notification that triggers AWS Config to run an evaluation for a rule. You canspecify the following notification types:
    /// * `ConfigurationItemChangeNotification` - Triggers an evaluation when AWS Config delivers a configuration item as a result of a resource change.
    /// * `OversizedConfigurationItemChangeNotification` - Triggers an evaluation when AWS Config delivers an oversized configuration item. AWS Config may generate this notification type when a resource changes and the notification exceeds the maximum size allowed by Amazon SNS.
    /// * `ScheduledNotification` - Triggers a periodic evaluation at the frequency specified for `maximum_execution_frequency`.
    /// * `ConfigurationSnapshotDeliveryCompleted` - Triggers a periodic evaluation when AWS Config delivers a configuration snapshot.
    #[builder(into, default)]
    #[serde(rename = "messageType")]
    pub r#message_type: Box<Option<String>>,
}
