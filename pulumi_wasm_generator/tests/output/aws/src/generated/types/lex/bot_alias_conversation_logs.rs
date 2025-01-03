#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct BotAliasConversationLogs {
    /// The Amazon Resource Name (ARN) of the IAM role used to write your logs to CloudWatch Logs or an S3 bucket. Must be between 20 and 2048 characters in length.
    #[builder(into)]
    #[serde(rename = "iamRoleArn")]
    pub r#iam_role_arn: Box<String>,
    /// The settings for your conversation logs. You can log text, audio, or both. Attributes are documented under log_settings.
    #[builder(into, default)]
    #[serde(rename = "logSettings")]
    pub r#log_settings: Box<Option<Vec<super::super::types::lex::BotAliasConversationLogsLogSetting>>>,
}
