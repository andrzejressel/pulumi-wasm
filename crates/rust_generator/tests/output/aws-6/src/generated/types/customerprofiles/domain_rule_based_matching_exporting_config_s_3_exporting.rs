#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct DomainRuleBasedMatchingExportingConfigS3Exporting {
    /// The name of the S3 bucket where Identity Resolution Jobs write result files.
    #[builder(into)]
    #[serde(rename = "s3BucketName")]
    pub r#s_3_bucket_name: Box<String>,
    /// The S3 key name of the location where Identity Resolution Jobs write result files.
    #[builder(into, default)]
    #[serde(rename = "s3KeyName")]
    pub r#s_3_key_name: Box<Option<String>>,
}
