#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetInstanceStorageConfigStorageConfigS3Config {
    /// The S3 bucket name.
    #[builder(into)]
    #[serde(rename = "bucketName")]
    pub r#bucket_name: Box<String>,
    /// The S3 bucket prefix.
    #[builder(into)]
    #[serde(rename = "bucketPrefix")]
    pub r#bucket_prefix: Box<String>,
    /// The encryption configuration. Documented below.
    #[builder(into)]
    #[serde(rename = "encryptionConfigs")]
    pub r#encryption_configs: Box<Vec<super::super::types::connect::GetInstanceStorageConfigStorageConfigS3ConfigEncryptionConfig>>,
}
