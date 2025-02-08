#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct ApplicationApplicationConfigurationSqlApplicationConfigurationOutput {
    /// Describes the data format when records are written to the destination.
    #[builder(into)]
    #[serde(rename = "destinationSchema")]
    pub r#destination_schema: Box<super::super::types::kinesisanalyticsv2::ApplicationApplicationConfigurationSqlApplicationConfigurationOutputDestinationSchema>,
    /// Identifies a Kinesis Data Firehose delivery stream as the destination.
    #[builder(into, default)]
    #[serde(rename = "kinesisFirehoseOutput")]
    pub r#kinesis_firehose_output: Box<Option<super::super::types::kinesisanalyticsv2::ApplicationApplicationConfigurationSqlApplicationConfigurationOutputKinesisFirehoseOutput>>,
    /// Identifies a Kinesis data stream as the destination.
    #[builder(into, default)]
    #[serde(rename = "kinesisStreamsOutput")]
    pub r#kinesis_streams_output: Box<Option<super::super::types::kinesisanalyticsv2::ApplicationApplicationConfigurationSqlApplicationConfigurationOutputKinesisStreamsOutput>>,
    /// Identifies a Lambda function as the destination.
    #[builder(into, default)]
    #[serde(rename = "lambdaOutput")]
    pub r#lambda_output: Box<Option<super::super::types::kinesisanalyticsv2::ApplicationApplicationConfigurationSqlApplicationConfigurationOutputLambdaOutput>>,
    /// The name of the in-application stream.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    #[builder(into, default)]
    #[serde(rename = "outputId")]
    pub r#output_id: Box<Option<String>>,
}
