#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetSnapshotSourceDiskEncryptionKey {
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
}
