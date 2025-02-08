#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ApplicationApplicationConfigurationApplicationCodeConfigurationCodeContentS3ContentLocation {
    /// The ARN for the S3 bucket containing the application code.
    #[builder(into)]
    #[serde(rename = "bucketArn")]
    pub r#bucket_arn: Box<String>,
    /// The file key for the object containing the application code.
    #[builder(into)]
    #[serde(rename = "fileKey")]
    pub r#file_key: Box<String>,
    /// The version of the object containing the application code.
    #[builder(into, default)]
    #[serde(rename = "objectVersion")]
    pub r#object_version: Box<Option<String>>,
}
