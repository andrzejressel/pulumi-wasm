#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct TopicRuleErrorActionCloudwatchAlarm {
    /// The CloudWatch alarm name.
    #[builder(into)]
    #[serde(rename = "alarmName")]
    pub r#alarm_name: Box<String>,
    /// The IAM role ARN that allows access to the CloudWatch alarm.
    #[builder(into)]
    #[serde(rename = "roleArn")]
    pub r#role_arn: Box<String>,
    /// The reason for the alarm change.
    #[builder(into)]
    #[serde(rename = "stateReason")]
    pub r#state_reason: Box<String>,
    /// The value of the alarm state. Acceptable values are: OK, ALARM, INSUFFICIENT_DATA.
    #[builder(into)]
    #[serde(rename = "stateValue")]
    pub r#state_value: Box<String>,
}
