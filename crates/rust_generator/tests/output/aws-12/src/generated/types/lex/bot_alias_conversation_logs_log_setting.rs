#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct BotAliasConversationLogsLogSetting {
    /// The destination where logs are delivered. Options are `CLOUDWATCH_LOGS` or `S3`.
    #[builder(into)]
    #[serde(rename = "destination")]
    pub r#destination: Box<String>,
    /// The Amazon Resource Name (ARN) of the key used to encrypt audio logs in an S3 bucket. This can only be specified when `destination` is set to `S3`. Must be between 20 and 2048 characters in length.
    #[builder(into, default)]
    #[serde(rename = "kmsKeyArn")]
    pub r#kms_key_arn: Box<Option<String>>,
    /// The type of logging that is enabled. Options are `AUDIO` or `TEXT`.
    #[builder(into)]
    #[serde(rename = "logType")]
    pub r#log_type: Box<String>,
    /// The Amazon Resource Name (ARN) of the CloudWatch Logs log group or S3 bucket where the logs are delivered. Must be less than or equal to 2048 characters in length.
    #[builder(into)]
    #[serde(rename = "resourceArn")]
    pub r#resource_arn: Box<String>,
    /// The prefix of the S3 object key for `AUDIO` logs or the log stream name for `TEXT` logs.
    #[builder(into, default)]
    #[serde(rename = "resourcePrefix")]
    pub r#resource_prefix: Box<Option<String>>,
}
