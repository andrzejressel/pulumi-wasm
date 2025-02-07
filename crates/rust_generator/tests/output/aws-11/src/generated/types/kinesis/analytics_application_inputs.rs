#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct AnalyticsApplicationInputs {
    /// The ARN of the Kinesis Analytics Application.
    #[builder(into, default)]
    #[serde(rename = "id")]
    pub r#id: Box<Option<String>>,
    /// The Kinesis Firehose configuration for the streaming source. Conflicts with `kinesis_stream`.
    /// See Kinesis Firehose below for more details.
    #[builder(into, default)]
    #[serde(rename = "kinesisFirehose")]
    pub r#kinesis_firehose: Box<Option<super::super::types::kinesis::AnalyticsApplicationInputsKinesisFirehose>>,
    /// The Kinesis Stream configuration for the streaming source. Conflicts with `kinesis_firehose`.
    /// See Kinesis Stream below for more details.
    #[builder(into, default)]
    #[serde(rename = "kinesisStream")]
    pub r#kinesis_stream: Box<Option<super::super::types::kinesis::AnalyticsApplicationInputsKinesisStream>>,
    /// The Name Prefix to use when creating an in-application stream.
    #[builder(into)]
    #[serde(rename = "namePrefix")]
    pub r#name_prefix: Box<String>,
    /// The number of Parallel in-application streams to create.
    /// See Parallelism below for more details.
    #[builder(into, default)]
    #[serde(rename = "parallelism")]
    pub r#parallelism: Box<Option<super::super::types::kinesis::AnalyticsApplicationInputsParallelism>>,
    /// The Processing Configuration to transform records as they are received from the stream.
    /// See Processing Configuration below for more details.
    #[builder(into, default)]
    #[serde(rename = "processingConfiguration")]
    pub r#processing_configuration: Box<Option<super::super::types::kinesis::AnalyticsApplicationInputsProcessingConfiguration>>,
    /// The Schema format of the data in the streaming source. See Source Schema below for more details.
    #[builder(into)]
    #[serde(rename = "schema")]
    pub r#schema: Box<super::super::types::kinesis::AnalyticsApplicationInputsSchema>,
    /// The point at which the application starts processing records from the streaming source.
    /// See Starting Position Configuration below for more details.
    #[builder(into, default)]
    #[serde(rename = "startingPositionConfigurations")]
    pub r#starting_position_configurations: Box<Option<Vec<super::super::types::kinesis::AnalyticsApplicationInputsStartingPositionConfiguration>>>,
    #[builder(into, default)]
    #[serde(rename = "streamNames")]
    pub r#stream_names: Box<Option<Vec<String>>>,
}
