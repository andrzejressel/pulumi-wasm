#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct WorkteamNotificationConfiguration {
    /// The ARN for the SNS topic to which notifications should be published.
    #[builder(into, default)]
    #[serde(rename = "notificationTopicArn")]
    pub r#notification_topic_arn: Box<Option<String>>,
}
