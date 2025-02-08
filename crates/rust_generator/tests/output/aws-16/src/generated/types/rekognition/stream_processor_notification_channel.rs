#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct StreamProcessorNotificationChannel {
    /// The Amazon Resource Number (ARN) of the Amazon Amazon Simple Notification Service topic to which Amazon Rekognition posts the completion status.
    #[builder(into, default)]
    #[serde(rename = "snsTopicArn")]
    pub r#sns_topic_arn: Box<Option<String>>,
}
