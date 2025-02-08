#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct InstanceStorageConfigStorageConfig {
    /// A block that specifies the configuration of the Kinesis Firehose delivery stream. Documented below.
    #[builder(into, default)]
    #[serde(rename = "kinesisFirehoseConfig")]
    pub r#kinesis_firehose_config: Box<Option<super::super::types::connect::InstanceStorageConfigStorageConfigKinesisFirehoseConfig>>,
    /// A block that specifies the configuration of the Kinesis data stream. Documented below.
    #[builder(into, default)]
    #[serde(rename = "kinesisStreamConfig")]
    pub r#kinesis_stream_config: Box<Option<super::super::types::connect::InstanceStorageConfigStorageConfigKinesisStreamConfig>>,
    /// A block that specifies the configuration of the Kinesis video stream. Documented below.
    #[builder(into, default)]
    #[serde(rename = "kinesisVideoStreamConfig")]
    pub r#kinesis_video_stream_config: Box<Option<super::super::types::connect::InstanceStorageConfigStorageConfigKinesisVideoStreamConfig>>,
    /// A block that specifies the configuration of S3 Bucket. Documented below.
    #[builder(into, default)]
    #[serde(rename = "s3Config")]
    pub r#s_3_config: Box<Option<super::super::types::connect::InstanceStorageConfigStorageConfigS3Config>>,
    /// A valid storage type. Valid Values: `S3` | `KINESIS_VIDEO_STREAM` | `KINESIS_STREAM` | `KINESIS_FIREHOSE`.
    #[builder(into)]
    #[serde(rename = "storageType")]
    pub r#storage_type: Box<String>,
}
