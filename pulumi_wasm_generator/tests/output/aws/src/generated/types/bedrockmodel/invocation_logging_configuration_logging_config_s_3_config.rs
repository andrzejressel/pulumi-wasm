#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct InvocationLoggingConfigurationLoggingConfigS3Config {
    /// S3 bucket name.
    #[builder(into, default)]
    #[serde(rename = "bucketName")]
    pub r#bucket_name: Box<Option<String>>,
    /// S3 prefix.
    #[builder(into, default)]
    #[serde(rename = "keyPrefix")]
    pub r#key_prefix: Box<Option<String>>,
}