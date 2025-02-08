#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct TopicRuleKinesis {
    /// The partition key.
    #[builder(into, default)]
    #[serde(rename = "partitionKey")]
    pub r#partition_key: Box<Option<String>>,
    /// The ARN of the IAM role that grants access to the Amazon Kinesis stream.
    #[builder(into)]
    #[serde(rename = "roleArn")]
    pub r#role_arn: Box<String>,
    /// The name of the Amazon Kinesis stream.
    #[builder(into)]
    #[serde(rename = "streamName")]
    pub r#stream_name: Box<String>,
}
