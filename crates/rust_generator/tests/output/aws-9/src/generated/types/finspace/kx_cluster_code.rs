#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct KxClusterCode {
    /// Unique name for the S3 bucket.
    #[builder(into)]
    #[serde(rename = "s3Bucket")]
    pub r#s_3_bucket: Box<String>,
    /// Full S3 path (excluding bucket) to the .zip file that contains the code to be loaded onto the cluster when it’s started.
    #[builder(into)]
    #[serde(rename = "s3Key")]
    pub r#s_3_key: Box<String>,
    /// Version of an S3 Object.
    #[builder(into, default)]
    #[serde(rename = "s3ObjectVersion")]
    pub r#s_3_object_version: Box<Option<String>>,
}
