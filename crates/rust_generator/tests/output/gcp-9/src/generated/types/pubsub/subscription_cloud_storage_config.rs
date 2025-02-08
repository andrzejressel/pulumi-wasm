#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct SubscriptionCloudStorageConfig {
    /// If set, message data will be written to Cloud Storage in Avro format.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "avroConfig")]
    pub r#avro_config: Box<Option<super::super::types::pubsub::SubscriptionCloudStorageConfigAvroConfig>>,
    /// User-provided name for the Cloud Storage bucket. The bucket must be created by the user. The bucket name must be without any prefix like "gs://".
    #[builder(into)]
    #[serde(rename = "bucket")]
    pub r#bucket: Box<String>,
    /// User-provided format string specifying how to represent datetimes in Cloud Storage filenames.
    #[builder(into, default)]
    #[serde(rename = "filenameDatetimeFormat")]
    pub r#filename_datetime_format: Box<Option<String>>,
    /// User-provided prefix for Cloud Storage filename.
    #[builder(into, default)]
    #[serde(rename = "filenamePrefix")]
    pub r#filename_prefix: Box<Option<String>>,
    /// User-provided suffix for Cloud Storage filename. Must not end in "/".
    #[builder(into, default)]
    #[serde(rename = "filenameSuffix")]
    pub r#filename_suffix: Box<Option<String>>,
    /// The maximum bytes that can be written to a Cloud Storage file before a new file is created. Min 1 KB, max 10 GiB.
    /// The maxBytes limit may be exceeded in cases where messages are larger than the limit.
    #[builder(into, default)]
    #[serde(rename = "maxBytes")]
    pub r#max_bytes: Box<Option<i32>>,
    /// The maximum duration that can elapse before a new Cloud Storage file is created. Min 1 minute, max 10 minutes, default 5 minutes.
    /// May not exceed the subscription's acknowledgement deadline.
    /// A duration in seconds with up to nine fractional digits, ending with 's'. Example: "3.5s".
    #[builder(into, default)]
    #[serde(rename = "maxDuration")]
    pub r#max_duration: Box<Option<String>>,
    /// The maximum messages that can be written to a Cloud Storage file before a new file is created. Min 1000 messages.
    #[builder(into, default)]
    #[serde(rename = "maxMessages")]
    pub r#max_messages: Box<Option<i32>>,
    /// The service account to use to write to Cloud Storage. If not specified, the Pub/Sub
    /// [service agent](https://cloud.google.com/iam/docs/service-agents),
    /// service-{project_number}@gcp-sa-pubsub.iam.gserviceaccount.com, is used.
    #[builder(into, default)]
    #[serde(rename = "serviceAccountEmail")]
    pub r#service_account_email: Box<Option<String>>,
    /// (Output)
    /// An output-only field that indicates whether or not the subscription can receive messages.
    #[builder(into, default)]
    #[serde(rename = "state")]
    pub r#state: Box<Option<String>>,
}
