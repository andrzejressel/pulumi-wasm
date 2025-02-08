#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct StreamProcessorOutput {
    /// The Amazon Kinesis Data Streams stream to which the Amazon Rekognition stream processor streams the analysis results. See `kinesis_data_stream`.
    #[builder(into, default)]
    #[serde(rename = "kinesisDataStream")]
    pub r#kinesis_data_stream: Box<Option<super::super::types::rekognition::StreamProcessorOutputKinesisDataStream>>,
    /// The Amazon S3 bucket location to which Amazon Rekognition publishes the detailed inference results of a video analysis operation. See `s3_destination`.
    #[builder(into, default)]
    #[serde(rename = "s3Destination")]
    pub r#s_3_destination: Box<Option<super::super::types::rekognition::StreamProcessorOutputS3Destination>>,
}
