#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct InstanceBootDisk {
    /// Whether the disk will be auto-deleted when the instance
    /// is deleted. Defaults to true.
    #[builder(into, default)]
    #[serde(rename = "autoDelete")]
    pub r#auto_delete: Box<Option<bool>>,
    /// Name with which attached disk will be accessible.
    /// On the instance, this device will be `/dev/disk/by-id/google-{{device_name}}`.
    #[builder(into, default)]
    #[serde(rename = "deviceName")]
    pub r#device_name: Box<Option<String>>,
    /// A 256-bit [customer-supplied encryption key]
    /// (https://cloud.google.com/compute/docs/disks/customer-supplied-encryption),
    /// encoded in [RFC 4648 base64](https://tools.ietf.org/html/rfc4648#section-4)
    /// to encrypt this disk. Only one of `kms_key_self_link` and `disk_encryption_key_raw`
    /// may be set.
    #[builder(into, default)]
    #[serde(rename = "diskEncryptionKeyRaw")]
    pub r#disk_encryption_key_raw: Box<Option<String>>,
    /// The [RFC 4648 base64](https://tools.ietf.org/html/rfc4648#section-4)
    /// encoded SHA-256 hash of the [customer-supplied encryption key]
    /// (https://cloud.google.com/compute/docs/disks/customer-supplied-encryption) that protects this resource.
    #[builder(into, default)]
    #[serde(rename = "diskEncryptionKeySha256")]
    pub r#disk_encryption_key_sha_256: Box<Option<String>>,
    /// Parameters for a new disk that will be created
    /// alongside the new instance. Either `initialize_params` or `source` must be set.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "initializeParams")]
    pub r#initialize_params: Box<Option<super::super::types::compute::InstanceBootDiskInitializeParams>>,
    /// The disk interface used for attaching this disk. One of SCSI or NVME. (This field is shared with attached_disk and only used for specific cases, please don't specify this field without advice from Google.)
    #[builder(into, default)]
    #[serde(rename = "interface")]
    pub r#interface: Box<Option<String>>,
    /// The self_link of the encryption key that is
    /// stored in Google Cloud KMS to encrypt this disk. Only one of `kms_key_self_link`
    /// and `disk_encryption_key_raw` may be set.
    #[builder(into, default)]
    #[serde(rename = "kmsKeySelfLink")]
    pub r#kms_key_self_link: Box<Option<String>>,
    /// The mode in which to attach this disk, either `READ_WRITE`
    /// or `READ_ONLY`. If not specified, the default is to attach the disk in `READ_WRITE` mode.
    #[builder(into, default)]
    #[serde(rename = "mode")]
    pub r#mode: Box<Option<String>>,
    /// The name or self_link of the existing disk (such as those managed by
    /// `gcp.compute.Disk`) or disk image. To create an instance from a snapshot, first create a
    /// `gcp.compute.Disk` from a snapshot and reference it here.
    #[builder(into, default)]
    #[serde(rename = "source")]
    pub r#source: Box<Option<String>>,
}
