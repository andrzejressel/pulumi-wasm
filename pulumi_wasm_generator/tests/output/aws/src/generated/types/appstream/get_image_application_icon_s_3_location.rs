#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetImageApplicationIconS3Location {
    /// S3 bucket of the S3 object.
    #[builder(into)]
    #[serde(rename = "s3Bucket")]
    pub r#s_3_bucket: Box<String>,
    /// S3 key of the S3 object.
    #[builder(into)]
    #[serde(rename = "s3Key")]
    pub r#s_3_key: Box<String>,
}