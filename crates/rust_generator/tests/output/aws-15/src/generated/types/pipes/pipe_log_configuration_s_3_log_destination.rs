#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct PipeLogConfigurationS3LogDestination {
    /// Name of the Amazon S3 bucket to which EventBridge delivers the log records for the pipe.
    #[builder(into)]
    #[serde(rename = "bucketName")]
    pub r#bucket_name: Box<String>,
    /// Amazon Web Services account that owns the Amazon S3 bucket to which EventBridge delivers the log records for the pipe.
    #[builder(into)]
    #[serde(rename = "bucketOwner")]
    pub r#bucket_owner: Box<String>,
    /// EventBridge format for the log records. Valid values `json`, `plain` and `w3c`.
    #[builder(into, default)]
    #[serde(rename = "outputFormat")]
    pub r#output_format: Box<Option<String>>,
    /// Prefix text with which to begin Amazon S3 log object names.
    #[builder(into, default)]
    #[serde(rename = "prefix")]
    pub r#prefix: Box<Option<String>>,
}
