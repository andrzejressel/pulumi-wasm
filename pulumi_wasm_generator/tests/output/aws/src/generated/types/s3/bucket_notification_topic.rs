#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct BucketNotificationTopic {
    /// [Event](http://docs.aws.amazon.com/AmazonS3/latest/dev/NotificationHowTo.html#notification-how-to-event-types-and-destinations) for which to send notifications.
    #[builder(into)]
    #[serde(rename = "events")]
    pub r#events: Box<Vec<String>>,
    /// Object key name prefix.
    #[builder(into, default)]
    #[serde(rename = "filterPrefix")]
    pub r#filter_prefix: Box<Option<String>>,
    /// Object key name suffix.
    #[builder(into, default)]
    #[serde(rename = "filterSuffix")]
    pub r#filter_suffix: Box<Option<String>>,
    /// Unique identifier for each of the notification configurations.
    #[builder(into, default)]
    #[serde(rename = "id")]
    pub r#id: Box<Option<String>>,
    /// SNS topic ARN.
    #[builder(into)]
    #[serde(rename = "topicArn")]
    pub r#topic_arn: Box<String>,
}
