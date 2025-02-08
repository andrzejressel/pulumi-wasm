#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct TaskTaskReportConfigS3Destination {
    /// Specifies the Amazon Resource Name (ARN) of the IAM policy that allows DataSync to upload a task report to your S3 bucket.
    #[builder(into)]
    #[serde(rename = "bucketAccessRoleArn")]
    pub r#bucket_access_role_arn: Box<String>,
    /// Specifies the ARN of the S3 bucket where DataSync uploads your report.
    #[builder(into)]
    #[serde(rename = "s3BucketArn")]
    pub r#s_3_bucket_arn: Box<String>,
    /// Specifies a bucket prefix for your report.
    #[builder(into, default)]
    #[serde(rename = "subdirectory")]
    pub r#subdirectory: Box<Option<String>>,
}
