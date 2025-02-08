#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct AnalyticsApplicationReferenceDataSourcesS3 {
    /// The S3 Bucket ARN.
    #[builder(into)]
    #[serde(rename = "bucketArn")]
    pub r#bucket_arn: Box<String>,
    /// The File Key name containing reference data.
    #[builder(into)]
    #[serde(rename = "fileKey")]
    pub r#file_key: Box<String>,
    /// The IAM Role ARN to read the data.
    #[builder(into)]
    #[serde(rename = "roleArn")]
    pub r#role_arn: Box<String>,
}
