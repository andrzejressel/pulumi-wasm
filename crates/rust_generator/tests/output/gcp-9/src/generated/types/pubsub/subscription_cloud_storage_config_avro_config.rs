#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct SubscriptionCloudStorageConfigAvroConfig {
    /// When true, the output Cloud Storage file will be serialized using the topic schema, if it exists.
    #[builder(into, default)]
    #[serde(rename = "useTopicSchema")]
    pub r#use_topic_schema: Box<Option<bool>>,
    /// When true, write the subscription name, messageId, publishTime, attributes, and orderingKey as additional fields in the output.
    #[builder(into, default)]
    #[serde(rename = "writeMetadata")]
    pub r#write_metadata: Box<Option<bool>>,
}
