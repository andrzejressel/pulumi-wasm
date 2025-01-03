#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct FirehoseDeliveryStreamKinesisSourceConfiguration {
    /// The kinesis stream used as the source of the firehose delivery stream.
    #[builder(into)]
    #[serde(rename = "kinesisStreamArn")]
    pub r#kinesis_stream_arn: Box<String>,
    /// The ARN of the role that provides access to the source Kinesis stream.
    #[builder(into)]
    #[serde(rename = "roleArn")]
    pub r#role_arn: Box<String>,
}
