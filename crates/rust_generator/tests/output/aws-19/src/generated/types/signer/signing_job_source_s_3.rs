#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct SigningJobSourceS3 {
    #[builder(into)]
    #[serde(rename = "bucket")]
    pub r#bucket: Box<String>,
    /// Key name of the object that contains your unsigned code.
    #[builder(into)]
    #[serde(rename = "key")]
    pub r#key: Box<String>,
    /// Version of your source image in your version enabled S3 bucket.
    #[builder(into)]
    #[serde(rename = "version")]
    pub r#version: Box<String>,
}
