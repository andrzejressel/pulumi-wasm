#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct TopicRuleIotAnalytic {
    /// The payload that contains a JSON array of records will be sent to IoT Analytics via a batch call.
    #[builder(into, default)]
    #[serde(rename = "batchMode")]
    pub r#batch_mode: Box<Option<bool>>,
    /// Name of AWS IOT Analytics channel.
    #[builder(into)]
    #[serde(rename = "channelName")]
    pub r#channel_name: Box<String>,
    /// The ARN of the IAM role that grants access.
    #[builder(into)]
    #[serde(rename = "roleArn")]
    pub r#role_arn: Box<String>,
}
