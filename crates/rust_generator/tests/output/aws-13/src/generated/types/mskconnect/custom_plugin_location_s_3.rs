#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct CustomPluginLocationS3 {
    /// The Amazon Resource Name (ARN) of an S3 bucket.
    #[builder(into)]
    #[serde(rename = "bucketArn")]
    pub r#bucket_arn: Box<String>,
    /// The file key for an object in an S3 bucket.
    #[builder(into)]
    #[serde(rename = "fileKey")]
    pub r#file_key: Box<String>,
    /// The version of an object in an S3 bucket.
    #[builder(into, default)]
    #[serde(rename = "objectVersion")]
    pub r#object_version: Box<Option<String>>,
}
