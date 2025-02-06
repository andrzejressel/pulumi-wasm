#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct InstanceAttachedDisk {
    /// Name with which the attached disk will be accessible
    /// under `/dev/disk/by-id/google-*`
    #[builder(into, default)]
    #[serde(rename = "deviceName")]
    pub r#device_name: Box<Option<String>>,
    /// A 256-bit [customer-supplied encryption key]
    /// (https://cloud.google.com/compute/docs/disks/customer-supplied-encryption),
    /// encoded in [RFC 4648 base64](https://tools.ietf.org/html/rfc4648#section-4)
    /// to encrypt this disk. Only one of `kms_key_self_link` and `disk_encryption_key_raw` may be set.
    #[builder(into, default)]
    #[serde(rename = "diskEncryptionKeyRaw")]
    pub r#disk_encryption_key_raw: Box<Option<String>>,
    /// The [RFC 4648 base64](https://tools.ietf.org/html/rfc4648#section-4)
    /// encoded SHA-256 hash of the [customer-supplied encryption key]
    /// (https://cloud.google.com/compute/docs/disks/customer-supplied-encryption) that protects this resource.
    #[builder(into, default)]
    #[serde(rename = "diskEncryptionKeySha256")]
    pub r#disk_encryption_key_sha_256: Box<Option<String>>,
    /// The self_link of the encryption key that is
    /// stored in Google Cloud KMS to encrypt this disk. Only one of `kms_key_self_link`
    /// and `disk_encryption_key_raw` may be set.
    #[builder(into, default)]
    #[serde(rename = "kmsKeySelfLink")]
    pub r#kms_key_self_link: Box<Option<String>>,
    /// Either "READ_ONLY" or "READ_WRITE", defaults to "READ_WRITE"
    /// If you have a persistent disk with data that you want to share
    /// between multiple instances, detach it from any read-write instances and
    /// attach it to one or more instances in read-only mode.
    #[builder(into, default)]
    #[serde(rename = "mode")]
    pub r#mode: Box<Option<String>>,
    /// The name or self_link of the disk to attach to this instance.
    #[builder(into)]
    #[serde(rename = "source")]
    pub r#source: Box<String>,
}
