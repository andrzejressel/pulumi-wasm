#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetTableMagneticStoreWritePropertyMagneticStoreRejectedDataLocationS3Configuration {
    /// Name of S3 bucket.
    #[builder(into)]
    #[serde(rename = "bucketName")]
    pub r#bucket_name: Box<String>,
    #[builder(into)]
    #[serde(rename = "encryptionOption")]
    pub r#encryption_option: Box<String>,
    /// AWS KMS key ID for S3 location with AWS maanged key.
    #[builder(into)]
    #[serde(rename = "kmsKeyId")]
    pub r#kms_key_id: Box<String>,
    /// Object key preview for S3 location.
    #[builder(into)]
    #[serde(rename = "objectKeyPrefix")]
    pub r#object_key_prefix: Box<String>,
}
