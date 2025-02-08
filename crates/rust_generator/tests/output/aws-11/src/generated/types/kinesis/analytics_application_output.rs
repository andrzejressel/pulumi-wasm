#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct AnalyticsApplicationOutput {
    /// The ARN of the Kinesis Analytics Application.
    #[builder(into, default)]
    #[serde(rename = "id")]
    pub r#id: Box<Option<String>>,
    /// The Kinesis Firehose configuration for the destination stream. Conflicts with `kinesis_stream`.
    /// See Kinesis Firehose below for more details.
    #[builder(into, default)]
    #[serde(rename = "kinesisFirehose")]
    pub r#kinesis_firehose: Box<Option<super::super::types::kinesis::AnalyticsApplicationOutputKinesisFirehose>>,
    /// The Kinesis Stream configuration for the destination stream. Conflicts with `kinesis_firehose`.
    /// See Kinesis Stream below for more details.
    #[builder(into, default)]
    #[serde(rename = "kinesisStream")]
    pub r#kinesis_stream: Box<Option<super::super::types::kinesis::AnalyticsApplicationOutputKinesisStream>>,
    /// The Lambda function destination. See Lambda below for more details.
    #[builder(into, default)]
    #[serde(rename = "lambda")]
    pub r#lambda: Box<Option<super::super::types::kinesis::AnalyticsApplicationOutputLambda>>,
    /// The Name of the in-application stream.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// The Schema format of the data written to the destination. See Destination Schema below for more details.
    #[builder(into)]
    #[serde(rename = "schema")]
    pub r#schema: Box<super::super::types::kinesis::AnalyticsApplicationOutputSchema>,
}
