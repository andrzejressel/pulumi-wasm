#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct SecurityConfigurationEncryptionConfigurationCloudwatchEncryption {
    /// Encryption mode to use for CloudWatch data. Valid values: `DISABLED`, `SSE-KMS`. Default value: `DISABLED`.
    #[builder(into, default)]
    #[serde(rename = "cloudwatchEncryptionMode")]
    pub r#cloudwatch_encryption_mode: Box<Option<String>>,
    /// Amazon Resource Name (ARN) of the KMS key to be used to encrypt the data.
    #[builder(into, default)]
    #[serde(rename = "kmsKeyArn")]
    pub r#kms_key_arn: Box<Option<String>>,
}