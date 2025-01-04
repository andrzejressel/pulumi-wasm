#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct MediaInsightsPipelineConfigurationElement {
    /// Configuration for Amazon Transcribe Call Analytics processor.
    #[builder(into, default)]
    #[serde(rename = "amazonTranscribeCallAnalyticsProcessorConfiguration")]
    pub r#amazon_transcribe_call_analytics_processor_configuration: Box<Option<super::super::types::chimesdkmediapipelines::MediaInsightsPipelineConfigurationElementAmazonTranscribeCallAnalyticsProcessorConfiguration>>,
    /// Configuration for Amazon Transcribe processor.
    #[builder(into, default)]
    #[serde(rename = "amazonTranscribeProcessorConfiguration")]
    pub r#amazon_transcribe_processor_configuration: Box<Option<super::super::types::chimesdkmediapipelines::MediaInsightsPipelineConfigurationElementAmazonTranscribeProcessorConfiguration>>,
    /// Configuration for Kinesis Data Stream sink.
    #[builder(into, default)]
    #[serde(rename = "kinesisDataStreamSinkConfiguration")]
    pub r#kinesis_data_stream_sink_configuration: Box<Option<super::super::types::chimesdkmediapipelines::MediaInsightsPipelineConfigurationElementKinesisDataStreamSinkConfiguration>>,
    /// Configuration for Lambda Function sink.
    #[builder(into, default)]
    #[serde(rename = "lambdaFunctionSinkConfiguration")]
    pub r#lambda_function_sink_configuration: Box<Option<super::super::types::chimesdkmediapipelines::MediaInsightsPipelineConfigurationElementLambdaFunctionSinkConfiguration>>,
    /// Configuration for S3 recording sink.
    #[builder(into, default)]
    #[serde(rename = "s3RecordingSinkConfiguration")]
    pub r#s_3_recording_sink_configuration: Box<Option<super::super::types::chimesdkmediapipelines::MediaInsightsPipelineConfigurationElementS3RecordingSinkConfiguration>>,
    /// Configuration for SNS Topic sink.
    #[builder(into, default)]
    #[serde(rename = "snsTopicSinkConfiguration")]
    pub r#sns_topic_sink_configuration: Box<Option<super::super::types::chimesdkmediapipelines::MediaInsightsPipelineConfigurationElementSnsTopicSinkConfiguration>>,
    /// Configuration for SQS Queue sink.
    #[builder(into, default)]
    #[serde(rename = "sqsQueueSinkConfiguration")]
    pub r#sqs_queue_sink_configuration: Box<Option<super::super::types::chimesdkmediapipelines::MediaInsightsPipelineConfigurationElementSqsQueueSinkConfiguration>>,
    /// Element type.
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: Box<String>,
    /// Configuration for Voice analytics processor.
    #[builder(into, default)]
    #[serde(rename = "voiceAnalyticsProcessorConfiguration")]
    pub r#voice_analytics_processor_configuration: Box<Option<super::super::types::chimesdkmediapipelines::MediaInsightsPipelineConfigurationElementVoiceAnalyticsProcessorConfiguration>>,
}
