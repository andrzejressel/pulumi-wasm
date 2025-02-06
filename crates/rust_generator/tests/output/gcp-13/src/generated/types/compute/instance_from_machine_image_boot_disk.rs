#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct InstanceFromMachineImageBootDisk {
    /// Whether the disk will be auto-deleted when the instance is deleted.
    #[builder(into, default)]
    #[serde(rename = "autoDelete")]
    pub r#auto_delete: Box<Option<bool>>,
    /// Name with which attached disk will be accessible under /dev/disk/by-id/
    #[builder(into, default)]
    #[serde(rename = "deviceName")]
    pub r#device_name: Box<Option<String>>,
    /// A 256-bit customer-supplied encryption key, encoded in RFC 4648 base64 to encrypt this disk. Only one of kms_key_self_link and disk_encryption_key_raw may be set.
    #[builder(into, default)]
    #[serde(rename = "diskEncryptionKeyRaw")]
    pub r#disk_encryption_key_raw: Box<Option<String>>,
    /// The RFC 4648 base64 encoded SHA-256 hash of the customer-supplied encryption key that protects this resource.
    #[builder(into, default)]
    #[serde(rename = "diskEncryptionKeySha256")]
    pub r#disk_encryption_key_sha_256: Box<Option<String>>,
    /// Parameters with which a disk was created alongside the instance.
    #[builder(into, default)]
    #[serde(rename = "initializeParams")]
    pub r#initialize_params: Box<Option<super::super::types::compute::InstanceFromMachineImageBootDiskInitializeParams>>,
    /// The disk interface used for attaching this disk. One of SCSI or NVME. (This field is shared with attached_disk and only used for specific cases, please don't specify this field without advice from Google.)
    #[builder(into, default)]
    #[serde(rename = "interface")]
    pub r#interface: Box<Option<String>>,
    /// The self_link of the encryption key that is stored in Google Cloud KMS to encrypt this disk. Only one of kms_key_self_link and disk_encryption_key_raw may be set.
    #[builder(into, default)]
    #[serde(rename = "kmsKeySelfLink")]
    pub r#kms_key_self_link: Box<Option<String>>,
    /// Read/write mode for the disk. One of "READ_ONLY" or "READ_WRITE".
    #[builder(into, default)]
    #[serde(rename = "mode")]
    pub r#mode: Box<Option<String>>,
    /// The name or self_link of the disk attached to this instance.
    #[builder(into, default)]
    #[serde(rename = "source")]
    pub r#source: Box<Option<String>>,
}
