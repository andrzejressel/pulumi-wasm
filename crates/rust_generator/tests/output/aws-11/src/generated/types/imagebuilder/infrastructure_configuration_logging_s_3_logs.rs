#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct InfrastructureConfigurationLoggingS3Logs {
    /// Name of the S3 Bucket.
    /// 
    /// The following arguments are optional:
    #[builder(into)]
    #[serde(rename = "s3BucketName")]
    pub r#s_3_bucket_name: Box<String>,
    /// Prefix to use for S3 logs. Defaults to `/`.
    #[builder(into, default)]
    #[serde(rename = "s3KeyPrefix")]
    pub r#s_3_key_prefix: Box<Option<String>>,
}
