#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetInstanceBootDisk {
    /// Whether the disk will be auto-deleted when the instance is deleted.
    #[builder(into)]
    #[serde(rename = "autoDelete")]
    pub r#auto_delete: Box<bool>,
    /// Name with which the attached disk is accessible
    /// under `/dev/disk/by-id/`
    #[builder(into)]
    #[serde(rename = "deviceName")]
    pub r#device_name: Box<String>,
    /// A 256-bit customer-supplied encryption key, encoded in RFC 4648 base64 to encrypt this disk. Only one of kms_key_self_link and disk_encryption_key_raw may be set.
    #[builder(into)]
    #[serde(rename = "diskEncryptionKeyRaw")]
    pub r#disk_encryption_key_raw: Box<String>,
    /// The [RFC 4648 base64](https://tools.ietf.org/html/rfc4648#section-4)
    /// encoded SHA-256 hash of the [customer-supplied encryption key]
    /// (<https://cloud.google.com/compute/docs/disks/customer-supplied-encryption>) that protects this resource.
    #[builder(into)]
    #[serde(rename = "diskEncryptionKeySha256")]
    pub r#disk_encryption_key_sha_256: Box<String>,
    /// Parameters with which a disk was created alongside the instance.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "initializeParams")]
    pub r#initialize_params: Box<Vec<super::super::types::compute::GetInstanceBootDiskInitializeParam>>,
    /// The disk interface used for attaching this disk. One of `SCSI` or `NVME`.
    #[builder(into)]
    #[serde(rename = "interface")]
    pub r#interface: Box<String>,
    /// The self_link of the encryption key that is stored in Google Cloud KMS to encrypt this disk. Only one of kms_key_self_link and disk_encryption_key_raw may be set.
    #[builder(into)]
    #[serde(rename = "kmsKeySelfLink")]
    pub r#kms_key_self_link: Box<String>,
    /// Read/write mode for the disk. One of `"READ_ONLY"` or `"READ_WRITE"`.
    #[builder(into)]
    #[serde(rename = "mode")]
    pub r#mode: Box<String>,
    /// The self_link of the disk attached to this instance.
    #[builder(into)]
    #[serde(rename = "source")]
    pub r#source: Box<String>,
}
