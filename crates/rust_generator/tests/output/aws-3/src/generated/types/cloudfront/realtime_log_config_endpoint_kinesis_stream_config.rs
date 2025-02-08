#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct RealtimeLogConfigEndpointKinesisStreamConfig {
    /// The ARN of an IAM role that CloudFront can use to send real-time log data to the Kinesis data stream.
    /// See the [AWS documentation](https://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/real-time-logs.html#understand-real-time-log-config-iam-role) for more information.
    #[builder(into)]
    #[serde(rename = "roleArn")]
    pub r#role_arn: Box<String>,
    /// The ARN of the Kinesis data stream.
    #[builder(into)]
    #[serde(rename = "streamArn")]
    pub r#stream_arn: Box<String>,
}
