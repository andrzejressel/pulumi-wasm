#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct DicomStoreNotificationConfig {
    /// The Cloud Pub/Sub topic that notifications of changes are published on. Supplied by the client.
    /// PubsubMessage.Data will contain the resource name. PubsubMessage.MessageId is the ID of this message.
    /// It is guaranteed to be unique within the topic. PubsubMessage.PublishTime is the time at which the message
    /// was published. Notifications are only sent if the topic is non-empty. Topic names must be scoped to a
    /// project. service-PROJECT_NUMBER@gcp-sa-healthcare.iam.gserviceaccount.com must have publisher permissions on the given
    /// Cloud Pub/Sub topic. Not having adequate permissions will cause the calls that send notifications to fail.
    #[builder(into)]
    #[serde(rename = "pubsubTopic")]
    pub r#pubsub_topic: Box<String>,
    /// Indicates whether or not to send Pub/Sub notifications on bulk import. Only supported for DICOM imports.
    #[builder(into, default)]
    #[serde(rename = "sendForBulkImport")]
    pub r#send_for_bulk_import: Box<Option<bool>>,
}
