#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct S3LocationS3Config {
    /// ARN of the IAM Role used to connect to the S3 Bucket.
    #[builder(into)]
    #[serde(rename = "bucketAccessRoleArn")]
    pub r#bucket_access_role_arn: Box<String>,
}
