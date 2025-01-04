#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ReplicatedVmManagedDiskTargetDiskEncryption {
    /// A `disk_encryption_key` block as defined below.
    #[builder(into)]
    #[serde(rename = "diskEncryptionKey")]
    pub r#disk_encryption_key: Box<super::super::types::siterecovery::ReplicatedVmManagedDiskTargetDiskEncryptionDiskEncryptionKey>,
    /// A `key_encryption_key` block as defined below.
    #[builder(into, default)]
    #[serde(rename = "keyEncryptionKey")]
    pub r#key_encryption_key: Box<Option<super::super::types::siterecovery::ReplicatedVmManagedDiskTargetDiskEncryptionKeyEncryptionKey>>,
}
