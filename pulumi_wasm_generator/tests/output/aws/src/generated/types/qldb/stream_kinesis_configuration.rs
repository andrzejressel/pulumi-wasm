#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct StreamKinesisConfiguration {
    /// Enables QLDB to publish multiple data records in a single Kinesis Data Streams record, increasing the number of records sent per API call. Default: `true`.
    #[builder(into, default)]
    #[serde(rename = "aggregationEnabled")]
    pub r#aggregation_enabled: Box<Option<bool>>,
    /// The Amazon Resource Name (ARN) of the Kinesis Data Streams resource.
    #[builder(into)]
    #[serde(rename = "streamArn")]
    pub r#stream_arn: Box<String>,
}