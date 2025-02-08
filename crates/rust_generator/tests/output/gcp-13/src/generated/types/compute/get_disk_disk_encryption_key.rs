#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetDiskDiskEncryptionKey {
    /// The self link of the encryption key used to encrypt the disk. Also called KmsKeyName
    /// in the cloud console. Your project's Compute Engine System service account
    /// ('service-{{PROJECT_NUMBER}}@compute-system.iam.gserviceaccount.com') must have
    /// 'roles/cloudkms.cryptoKeyEncrypterDecrypter' to use this feature.
    /// See https://cloud.google.com/compute/docs/disks/customer-managed-encryption#encrypt_a_new_persistent_disk_with_your_own_keys
    #[builder(into)]
    #[serde(rename = "kmsKeySelfLink")]
    pub r#kms_key_self_link: Box<String>,
    /// The service account used for the encryption request for the given KMS key.
    /// If absent, the Compute Engine Service Agent service account is used.
    #[builder(into)]
    #[serde(rename = "kmsKeyServiceAccount")]
    pub r#kms_key_service_account: Box<String>,
    /// Specifies a 256-bit customer-supplied encryption key, encoded in
    /// RFC 4648 base64 to either encrypt or decrypt this resource.
    #[builder(into)]
    #[serde(rename = "rawKey")]
    pub r#raw_key: Box<String>,
    /// Specifies an RFC 4648 base64 encoded, RSA-wrapped 2048-bit
    /// customer-supplied encryption key to either encrypt or decrypt
    /// this resource. You can provide either the rawKey or the rsaEncryptedKey.
    #[builder(into)]
    #[serde(rename = "rsaEncryptedKey")]
    pub r#rsa_encrypted_key: Box<String>,
    /// The RFC 4648 base64 encoded SHA-256 hash of the customer-supplied
    /// encryption key that protects this resource.
    #[builder(into)]
    #[serde(rename = "sha256")]
    pub r#sha_256: Box<String>,
}
