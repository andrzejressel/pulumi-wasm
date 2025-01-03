#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct S3LocationS3Config {
    /// ARN of the IAM Role used to connect to the S3 Bucket.
    #[builder(into)]
    #[serde(rename = "bucketAccessRoleArn")]
    pub r#bucket_access_role_arn: Box<String>,
}
