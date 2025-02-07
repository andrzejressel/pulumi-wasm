#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetEnvironmentMonitor {
    /// ARN of the Amazon CloudWatch alarm.
    #[builder(into)]
    #[serde(rename = "alarmArn")]
    pub r#alarm_arn: Box<String>,
    /// ARN of an IAM role for AWS AppConfig to monitor.
    #[builder(into)]
    #[serde(rename = "alarmRoleArn")]
    pub r#alarm_role_arn: Box<String>,
}
