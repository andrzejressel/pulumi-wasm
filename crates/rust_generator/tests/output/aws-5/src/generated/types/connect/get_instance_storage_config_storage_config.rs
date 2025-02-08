#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct GetInstanceStorageConfigStorageConfig {
    /// A block that specifies the configuration of the Kinesis Firehose delivery stream. Documented below.
    #[builder(into)]
    #[serde(rename = "kinesisFirehoseConfigs")]
    pub r#kinesis_firehose_configs: Box<Vec<super::super::types::connect::GetInstanceStorageConfigStorageConfigKinesisFirehoseConfig>>,
    /// A block that specifies the configuration of the Kinesis data stream. Documented below.
    #[builder(into)]
    #[serde(rename = "kinesisStreamConfigs")]
    pub r#kinesis_stream_configs: Box<Vec<super::super::types::connect::GetInstanceStorageConfigStorageConfigKinesisStreamConfig>>,
    /// A block that specifies the configuration of the Kinesis video stream. Documented below.
    #[builder(into)]
    #[serde(rename = "kinesisVideoStreamConfigs")]
    pub r#kinesis_video_stream_configs: Box<Vec<super::super::types::connect::GetInstanceStorageConfigStorageConfigKinesisVideoStreamConfig>>,
    /// A block that specifies the configuration of S3 Bucket. Documented below.
    #[builder(into)]
    #[serde(rename = "s3Configs")]
    pub r#s_3_configs: Box<Vec<super::super::types::connect::GetInstanceStorageConfigStorageConfigS3Config>>,
    /// A valid storage type. Valid Values: `S3` | `KINESIS_VIDEO_STREAM` | `KINESIS_STREAM` | `KINESIS_FIREHOSE`.
    #[builder(into)]
    #[serde(rename = "storageType")]
    pub r#storage_type: Box<String>,
}
