#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GraphQlApiLogConfig {
    /// Amazon Resource Name of the service role that AWS AppSync will assume to publish to Amazon CloudWatch logs in your account.
    #[builder(into)]
    #[serde(rename = "cloudwatchLogsRoleArn")]
    pub r#cloudwatch_logs_role_arn: Box<String>,
    /// Set to TRUE to exclude sections that contain information such as headers, context, and evaluated mapping templates, regardless of logging  level. Valid values: `true`, `false`. Default value: `false`
    #[builder(into, default)]
    #[serde(rename = "excludeVerboseContent")]
    pub r#exclude_verbose_content: Box<Option<bool>>,
    /// Field logging level. Valid values: `ALL`, `ERROR`, `NONE`.
    #[builder(into)]
    #[serde(rename = "fieldLogLevel")]
    pub r#field_log_level: Box<String>,
}
