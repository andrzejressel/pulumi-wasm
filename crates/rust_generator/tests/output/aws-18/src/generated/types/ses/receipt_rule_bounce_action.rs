#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ReceiptRuleBounceAction {
    /// The message to send
    #[builder(into)]
    #[serde(rename = "message")]
    pub r#message: Box<String>,
    /// The position of the action in the receipt rule
    #[builder(into)]
    #[serde(rename = "position")]
    pub r#position: Box<i32>,
    /// The email address of the sender
    #[builder(into)]
    #[serde(rename = "sender")]
    pub r#sender: Box<String>,
    /// The RFC 5321 SMTP reply code
    #[builder(into)]
    #[serde(rename = "smtpReplyCode")]
    pub r#smtp_reply_code: Box<String>,
    /// The RFC 3463 SMTP enhanced status code
    #[builder(into, default)]
    #[serde(rename = "statusCode")]
    pub r#status_code: Box<Option<String>>,
    /// The ARN of an SNS topic to notify
    #[builder(into, default)]
    #[serde(rename = "topicArn")]
    pub r#topic_arn: Box<Option<String>>,
}
