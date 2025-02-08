#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct InstanceStorageConfigStorageConfigKinesisVideoStreamConfig {
    /// The encryption configuration. Documented below.
    #[builder(into)]
    #[serde(rename = "encryptionConfig")]
    pub r#encryption_config: Box<super::super::types::connect::InstanceStorageConfigStorageConfigKinesisVideoStreamConfigEncryptionConfig>,
    /// The prefix of the video stream. Minimum length of `1`. Maximum length of `128`. When read from the state, the value returned is `<prefix>-connect-<connect_instance_alias>-contact-` since the API appends additional details to the `prefix`.
    #[builder(into)]
    #[serde(rename = "prefix")]
    pub r#prefix: Box<String>,
    /// The number of hours data is retained in the stream. Kinesis Video Streams retains the data in a data store that is associated with the stream. Minimum value of `0`. Maximum value of `87600`. A value of `0`, indicates that the stream does not persist data.
    #[builder(into)]
    #[serde(rename = "retentionPeriodHours")]
    pub r#retention_period_hours: Box<i32>,
}
