#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ApplicationApplicationConfigurationSqlApplicationConfigurationInput {
    #[builder(into, default)]
    #[serde(rename = "inAppStreamNames")]
    pub r#in_app_stream_names: Box<Option<Vec<String>>>,
    #[builder(into, default)]
    #[serde(rename = "inputId")]
    pub r#input_id: Box<Option<String>>,
    /// Describes the number of in-application streams to create.
    #[builder(into, default)]
    #[serde(rename = "inputParallelism")]
    pub r#input_parallelism: Box<Option<super::super::types::kinesisanalyticsv2::ApplicationApplicationConfigurationSqlApplicationConfigurationInputInputParallelism>>,
    /// The input processing configuration for the input.
    /// An input processor transforms records as they are received from the stream, before the application's SQL code executes.
    #[builder(into, default)]
    #[serde(rename = "inputProcessingConfiguration")]
    pub r#input_processing_configuration: Box<Option<super::super::types::kinesisanalyticsv2::ApplicationApplicationConfigurationSqlApplicationConfigurationInputInputProcessingConfiguration>>,
    /// Describes the format of the data in the streaming source, and how each data element maps to corresponding columns in the in-application stream that is being created.
    #[builder(into)]
    #[serde(rename = "inputSchema")]
    pub r#input_schema: Box<super::super::types::kinesisanalyticsv2::ApplicationApplicationConfigurationSqlApplicationConfigurationInputInputSchema>,
    /// The point at which the application starts processing records from the streaming source.
    #[builder(into, default)]
    #[serde(rename = "inputStartingPositionConfigurations")]
    pub r#input_starting_position_configurations: Box<Option<Vec<super::super::types::kinesisanalyticsv2::ApplicationApplicationConfigurationSqlApplicationConfigurationInputInputStartingPositionConfiguration>>>,
    /// If the streaming source is a Kinesis Data Firehose delivery stream, identifies the delivery stream's ARN.
    #[builder(into, default)]
    #[serde(rename = "kinesisFirehoseInput")]
    pub r#kinesis_firehose_input: Box<Option<super::super::types::kinesisanalyticsv2::ApplicationApplicationConfigurationSqlApplicationConfigurationInputKinesisFirehoseInput>>,
    /// If the streaming source is a Kinesis data stream, identifies the stream's Amazon Resource Name (ARN).
    #[builder(into, default)]
    #[serde(rename = "kinesisStreamsInput")]
    pub r#kinesis_streams_input: Box<Option<super::super::types::kinesisanalyticsv2::ApplicationApplicationConfigurationSqlApplicationConfigurationInputKinesisStreamsInput>>,
    /// The name prefix to use when creating an in-application stream.
    #[builder(into)]
    #[serde(rename = "namePrefix")]
    pub r#name_prefix: Box<String>,
}
