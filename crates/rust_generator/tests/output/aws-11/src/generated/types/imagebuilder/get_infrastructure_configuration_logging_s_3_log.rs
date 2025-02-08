#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetInfrastructureConfigurationLoggingS3Log {
    /// Name of the S3 Bucket for logging.
    #[builder(into)]
    #[serde(rename = "s3BucketName")]
    pub r#s_3_bucket_name: Box<String>,
    /// Key prefix for S3 Bucket logging.
    #[builder(into)]
    #[serde(rename = "s3KeyPrefix")]
    pub r#s_3_key_prefix: Box<String>,
}
