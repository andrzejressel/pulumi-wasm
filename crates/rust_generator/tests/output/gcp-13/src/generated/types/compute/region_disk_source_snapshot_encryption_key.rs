#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct RegionDiskSourceSnapshotEncryptionKey {
    /// The name of the encryption key that is stored in Google Cloud KMS.
    #[builder(into, default)]
    #[serde(rename = "kmsKeyName")]
    pub r#kms_key_name: Box<Option<String>>,
    /// Specifies a 256-bit customer-supplied encryption key, encoded in
    /// RFC 4648 base64 to either encrypt or decrypt this resource.
    #[builder(into, default)]
    #[serde(rename = "rawKey")]
    pub r#raw_key: Box<Option<String>>,
    /// (Output)
    /// The RFC 4648 base64 encoded SHA-256 hash of the customer-supplied
    /// encryption key that protects this resource.
    #[builder(into, default)]
    #[serde(rename = "sha256")]
    pub r#sha_256: Box<Option<String>>,
}
