#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct FeatureGroupOfflineStoreConfigS3StorageConfig {
    /// The AWS Key Management Service (KMS) key ID of the key used to encrypt any objects written into the OfflineStore S3 location.
    #[builder(into, default)]
    #[serde(rename = "kmsKeyId")]
    pub r#kms_key_id: Box<Option<String>>,
    /// The S3 path where offline records are written.
    #[builder(into, default)]
    #[serde(rename = "resolvedOutputS3Uri")]
    pub r#resolved_output_s_3_uri: Box<Option<String>>,
    /// The S3 URI, or location in Amazon S3, of OfflineStore.
    #[builder(into)]
    #[serde(rename = "s3Uri")]
    pub r#s_3_uri: Box<String>,
}