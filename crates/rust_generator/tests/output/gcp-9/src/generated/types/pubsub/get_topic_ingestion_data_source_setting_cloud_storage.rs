#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetTopicIngestionDataSourceSettingCloudStorage {
    /// Configuration for reading Cloud Storage data in Avro binary format. The
    /// bytes of each object will be set to the 'data' field of a Pub/Sub message.
    #[builder(into)]
    #[serde(rename = "avroFormats")]
    pub r#avro_formats: Box<Vec<super::super::types::pubsub::GetTopicIngestionDataSourceSettingCloudStorageAvroFormat>>,
    /// Cloud Storage bucket. The bucket name must be without any
    /// prefix like "gs://". See the bucket naming requirements:
    /// https://cloud.google.com/storage/docs/buckets#naming.
    #[builder(into)]
    #[serde(rename = "bucket")]
    pub r#bucket: Box<String>,
    /// Glob pattern used to match objects that will be ingested. If unset, all
    /// objects will be ingested. See the supported patterns:
    /// https://cloud.google.com/storage/docs/json_api/v1/objects/list#list-objects-and-prefixes-using-glob
    #[builder(into)]
    #[serde(rename = "matchGlob")]
    pub r#match_glob: Box<String>,
    /// The timestamp set in RFC3339 text format. If set, only objects with a
    /// larger or equal timestamp will be ingested. Unset by default, meaning
    /// all objects will be ingested.
    #[builder(into)]
    #[serde(rename = "minimumObjectCreateTime")]
    pub r#minimum_object_create_time: Box<String>,
    /// Configuration for reading Cloud Storage data written via Cloud Storage
    /// subscriptions(See https://cloud.google.com/pubsub/docs/cloudstorage). The
    /// data and attributes fields of the originally exported Pub/Sub message
    /// will be restored when publishing.
    #[builder(into)]
    #[serde(rename = "pubsubAvroFormats")]
    pub r#pubsub_avro_formats: Box<Vec<super::super::types::pubsub::GetTopicIngestionDataSourceSettingCloudStoragePubsubAvroFormat>>,
    /// Configuration for reading Cloud Storage data in text format. Each line of
    /// text as specified by the delimiter will be set to the 'data' field of a
    /// Pub/Sub message.
    #[builder(into)]
    #[serde(rename = "textFormats")]
    pub r#text_formats: Box<Vec<super::super::types::pubsub::GetTopicIngestionDataSourceSettingCloudStorageTextFormat>>,
}
