#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct TransferJobNotificationConfig {
    /// Event types for which a notification is desired. If empty, send notifications for all event types. The valid types are "TRANSFER_OPERATION_SUCCESS", "TRANSFER_OPERATION_FAILED", "TRANSFER_OPERATION_ABORTED".
    #[builder(into, default)]
    #[serde(rename = "eventTypes")]
    pub r#event_types: Box<Option<Vec<String>>>,
    /// The desired format of the notification message payloads. One of "NONE" or "JSON".
    #[builder(into)]
    #[serde(rename = "payloadFormat")]
    pub r#payload_format: Box<String>,
    /// The Topic.name of the Pub/Sub topic to which to publish notifications. Must be of the format: projects/{project}/topics/{topic}. Not matching this format results in an INVALID_ARGUMENT error.
    #[builder(into)]
    #[serde(rename = "pubsubTopic")]
    pub r#pubsub_topic: Box<String>,
}
