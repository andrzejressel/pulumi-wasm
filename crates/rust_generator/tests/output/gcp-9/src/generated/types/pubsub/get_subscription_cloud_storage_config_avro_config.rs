#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct GetSubscriptionCloudStorageConfigAvroConfig {
    /// When true, the output Cloud Storage file will be serialized using the topic schema, if it exists.
    #[builder(into)]
    #[serde(rename = "useTopicSchema")]
    pub r#use_topic_schema: Box<bool>,
    /// When true, write the subscription name, messageId, publishTime, attributes, and orderingKey as additional fields in the output.
    #[builder(into)]
    #[serde(rename = "writeMetadata")]
    pub r#write_metadata: Box<bool>,
}
