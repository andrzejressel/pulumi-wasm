#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct MaintenanceWindowTaskTaskInvocationParametersRunCommandParametersCloudwatchConfig {
    /// The name of the CloudWatch log group where you want to send command output. If you don't specify a group name, Systems Manager automatically creates a log group for you. The log group uses the following naming format: aws/ssm/SystemsManagerDocumentName.
    #[builder(into, default)]
    #[serde(rename = "cloudwatchLogGroupName")]
    pub r#cloudwatch_log_group_name: Box<Option<String>>,
    /// Enables Systems Manager to send command output to CloudWatch Logs.
    #[builder(into, default)]
    #[serde(rename = "cloudwatchOutputEnabled")]
    pub r#cloudwatch_output_enabled: Box<Option<bool>>,
}
