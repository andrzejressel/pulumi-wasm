#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct TopicRuleCloudwatchLog {
    /// The payload that contains a JSON array of records will be sent to CloudWatch via a batch call.
    #[builder(into, default)]
    #[serde(rename = "batchMode")]
    pub r#batch_mode: Box<Option<bool>>,
    /// The CloudWatch log group name.
    #[builder(into)]
    #[serde(rename = "logGroupName")]
    pub r#log_group_name: Box<String>,
    /// The IAM role ARN that allows access to the CloudWatch alarm.
    #[builder(into)]
    #[serde(rename = "roleArn")]
    pub r#role_arn: Box<String>,
}
