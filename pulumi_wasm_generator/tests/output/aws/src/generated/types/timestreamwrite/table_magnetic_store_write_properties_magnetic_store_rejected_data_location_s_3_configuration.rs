#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct TableMagneticStoreWritePropertiesMagneticStoreRejectedDataLocationS3Configuration {
    /// Bucket name of the customer S3 bucket.
    #[builder(into, default)]
    #[serde(rename = "bucketName")]
    pub r#bucket_name: Box<Option<String>>,
    /// Encryption option for the customer s3 location. Options are S3 server side encryption with an S3-managed key or KMS managed key. Valid values are `SSE_KMS` and `SSE_S3`.
    #[builder(into, default)]
    #[serde(rename = "encryptionOption")]
    pub r#encryption_option: Box<Option<String>>,
    /// KMS key arn for the customer s3 location when encrypting with a KMS managed key.
    #[builder(into, default)]
    #[serde(rename = "kmsKeyId")]
    pub r#kms_key_id: Box<Option<String>>,
    /// Object key prefix for the customer S3 location.
    #[builder(into, default)]
    #[serde(rename = "objectKeyPrefix")]
    pub r#object_key_prefix: Box<Option<String>>,
}
