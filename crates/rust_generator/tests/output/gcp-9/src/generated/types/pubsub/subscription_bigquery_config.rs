#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct SubscriptionBigqueryConfig {
    /// When true and use_topic_schema or use_table_schema is true, any fields that are a part of the topic schema or message schema that
    /// are not part of the BigQuery table schema are dropped when writing to BigQuery. Otherwise, the schemas must be kept in sync
    /// and any messages with extra fields are not written and remain in the subscription's backlog.
    #[builder(into, default)]
    #[serde(rename = "dropUnknownFields")]
    pub r#drop_unknown_fields: Box<Option<bool>>,
    /// The service account to use to write to BigQuery. If not specified, the Pub/Sub
    /// [service agent](https://cloud.google.com/iam/docs/service-agents),
    /// service-{project_number}@gcp-sa-pubsub.iam.gserviceaccount.com, is used.
    #[builder(into, default)]
    #[serde(rename = "serviceAccountEmail")]
    pub r#service_account_email: Box<Option<String>>,
    /// The name of the table to which to write data, of the form {projectId}.{datasetId}.{tableId}
    #[builder(into)]
    #[serde(rename = "table")]
    pub r#table: Box<String>,
    /// When true, use the BigQuery table's schema as the columns to write to in BigQuery. Messages
    /// must be published in JSON format. Only one of use_topic_schema and use_table_schema can be set.
    #[builder(into, default)]
    #[serde(rename = "useTableSchema")]
    pub r#use_table_schema: Box<Option<bool>>,
    /// When true, use the topic's schema as the columns to write to in BigQuery, if it exists.
    /// Only one of use_topic_schema and use_table_schema can be set.
    #[builder(into, default)]
    #[serde(rename = "useTopicSchema")]
    pub r#use_topic_schema: Box<Option<bool>>,
    /// When true, write the subscription name, messageId, publishTime, attributes, and orderingKey to additional columns in the table.
    /// The subscription name, messageId, and publishTime fields are put in their own columns while all other message properties (other than data) are written to a JSON object in the attributes column.
    #[builder(into, default)]
    #[serde(rename = "writeMetadata")]
    pub r#write_metadata: Box<Option<bool>>,
}
