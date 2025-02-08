#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct TopicRuleErrorActionIotEvents {
    /// The payload that contains a JSON array of records will be sent to IoT Events via a batch call.
    #[builder(into, default)]
    #[serde(rename = "batchMode")]
    pub r#batch_mode: Box<Option<bool>>,
    /// The name of the AWS IoT Events input.
    #[builder(into)]
    #[serde(rename = "inputName")]
    pub r#input_name: Box<String>,
    /// Use this to ensure that only one input (message) with a given messageId is processed by an AWS IoT Events detector.
    #[builder(into, default)]
    #[serde(rename = "messageId")]
    pub r#message_id: Box<Option<String>>,
    /// The ARN of the IAM role that grants access.
    #[builder(into)]
    #[serde(rename = "roleArn")]
    pub r#role_arn: Box<String>,
}
