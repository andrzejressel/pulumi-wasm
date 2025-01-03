#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct LoggingConfigurationDestinationConfigurationFirehose {
    /// Name of the Amazon Kinesis Firehose delivery stream where chat activity will be logged.
    #[builder(into)]
    #[serde(rename = "deliveryStreamName")]
    pub r#delivery_stream_name: Box<String>,
}
