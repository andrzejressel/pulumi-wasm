#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct TopicRuleErrorActionFirehose {
    /// The payload that contains a JSON array of records will be sent to Kinesis Firehose via a batch call.
    #[builder(into, default)]
    #[serde(rename = "batchMode")]
    pub r#batch_mode: Box<Option<bool>>,
    /// The delivery stream name.
    #[builder(into)]
    #[serde(rename = "deliveryStreamName")]
    pub r#delivery_stream_name: Box<String>,
    /// The IAM role ARN that grants access to the Amazon Kinesis Firehose stream.
    #[builder(into)]
    #[serde(rename = "roleArn")]
    pub r#role_arn: Box<String>,
    /// A character separator that is used to separate records written to the Firehose stream. Valid values are: '\n' (newline), '\t' (tab), '\r\n' (Windows newline), ',' (comma).
    #[builder(into, default)]
    #[serde(rename = "separator")]
    pub r#separator: Box<Option<String>>,
}
