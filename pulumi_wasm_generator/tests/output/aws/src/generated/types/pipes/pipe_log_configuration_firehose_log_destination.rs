#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct PipeLogConfigurationFirehoseLogDestination {
    /// Amazon Resource Name (ARN) of the Kinesis Data Firehose delivery stream to which EventBridge delivers the pipe log records.
    #[builder(into)]
    #[serde(rename = "deliveryStreamArn")]
    pub r#delivery_stream_arn: Box<String>,
}