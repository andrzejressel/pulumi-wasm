#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct TopicRuleSqs {
    /// The URL of the Amazon SQS queue.
    #[builder(into)]
    #[serde(rename = "queueUrl")]
    pub r#queue_url: Box<String>,
    /// The ARN of the IAM role that grants access.
    #[builder(into)]
    #[serde(rename = "roleArn")]
    pub r#role_arn: Box<String>,
    /// Specifies whether to use Base64 encoding.
    #[builder(into)]
    #[serde(rename = "useBase64")]
    pub r#use_base_64: Box<bool>,
}
