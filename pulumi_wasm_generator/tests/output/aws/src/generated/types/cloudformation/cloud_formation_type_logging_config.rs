#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct CloudFormationTypeLoggingConfig {
    /// Name of the CloudWatch Log Group where CloudFormation sends error logging information when invoking the type's handlers.
    #[builder(into)]
    #[serde(rename = "logGroupName")]
    pub r#log_group_name: Box<String>,
    /// Amazon Resource Name (ARN) of the IAM Role CloudFormation assumes when sending error logging information to CloudWatch Logs.
    #[builder(into)]
    #[serde(rename = "logRoleArn")]
    pub r#log_role_arn: Box<String>,
}