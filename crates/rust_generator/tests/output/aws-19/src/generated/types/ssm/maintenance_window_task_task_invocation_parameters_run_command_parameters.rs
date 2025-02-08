#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct MaintenanceWindowTaskTaskInvocationParametersRunCommandParameters {
    /// Configuration options for sending command output to CloudWatch Logs. Documented below.
    #[builder(into, default)]
    #[serde(rename = "cloudwatchConfig")]
    pub r#cloudwatch_config: Box<Option<super::super::types::ssm::MaintenanceWindowTaskTaskInvocationParametersRunCommandParametersCloudwatchConfig>>,
    /// Information about the command(s) to execute.
    #[builder(into, default)]
    #[serde(rename = "comment")]
    pub r#comment: Box<Option<String>>,
    /// The SHA-256 or SHA-1 hash created by the system when the document was created. SHA-1 hashes have been deprecated.
    #[builder(into, default)]
    #[serde(rename = "documentHash")]
    pub r#document_hash: Box<Option<String>>,
    /// SHA-256 or SHA-1. SHA-1 hashes have been deprecated. Valid values: `Sha256` and `Sha1`
    #[builder(into, default)]
    #[serde(rename = "documentHashType")]
    pub r#document_hash_type: Box<Option<String>>,
    /// The version of an Automation document to use during task execution.
    #[builder(into, default)]
    #[serde(rename = "documentVersion")]
    pub r#document_version: Box<Option<String>>,
    /// Configurations for sending notifications about command status changes on a per-instance basis. Documented below.
    #[builder(into, default)]
    #[serde(rename = "notificationConfig")]
    pub r#notification_config: Box<Option<super::super::types::ssm::MaintenanceWindowTaskTaskInvocationParametersRunCommandParametersNotificationConfig>>,
    /// The name of the Amazon S3 bucket.
    #[builder(into, default)]
    #[serde(rename = "outputS3Bucket")]
    pub r#output_s_3_bucket: Box<Option<String>>,
    /// The Amazon S3 bucket subfolder.
    #[builder(into, default)]
    #[serde(rename = "outputS3KeyPrefix")]
    pub r#output_s_3_key_prefix: Box<Option<String>>,
    /// The parameters for the RUN_COMMAND task execution. Documented below.
    #[builder(into, default)]
    #[serde(rename = "parameters")]
    pub r#parameters: Box<Option<Vec<super::super::types::ssm::MaintenanceWindowTaskTaskInvocationParametersRunCommandParametersParameter>>>,
    /// The Amazon Resource Name (ARN) of the AWS Identity and Access Management (IAM) service role to use to publish Amazon Simple Notification Service (Amazon SNS) notifications for maintenance window Run Command tasks.
    #[builder(into, default)]
    #[serde(rename = "serviceRoleArn")]
    pub r#service_role_arn: Box<Option<String>>,
    /// If this time is reached and the command has not already started executing, it doesn't run.
    #[builder(into, default)]
    #[serde(rename = "timeoutSeconds")]
    pub r#timeout_seconds: Box<Option<i32>>,
}
