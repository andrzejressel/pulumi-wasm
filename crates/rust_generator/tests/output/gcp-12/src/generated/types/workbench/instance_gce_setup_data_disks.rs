#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct InstanceGceSetupDataDisks {
    /// Optional. Input only. Disk encryption method used on the boot
    /// and data disks, defaults to GMEK.
    /// Possible values are: `GMEK`, `CMEK`.
    #[builder(into, default)]
    #[serde(rename = "diskEncryption")]
    pub r#disk_encryption: Box<Option<String>>,
    /// Optional. The size of the disk in GB attached to this VM instance,
    /// up to a maximum of 64000 GB (64 TB). If not specified, this defaults to
    /// 100.
    #[builder(into, default)]
    #[serde(rename = "diskSizeGb")]
    pub r#disk_size_gb: Box<Option<String>>,
    /// Optional. Input only. Indicates the type of the disk.
    /// Possible values are: `PD_STANDARD`, `PD_SSD`, `PD_BALANCED`, `PD_EXTREME`.
    #[builder(into, default)]
    #[serde(rename = "diskType")]
    pub r#disk_type: Box<Option<String>>,
    /// 'Optional. The KMS key used to encrypt the disks,
    /// only applicable if disk_encryption is CMEK. Format: `projects/{project_id}/locations/{location}/keyRings/{key_ring_id}/cryptoKeys/{key_id}`
    /// Learn more about using your own encryption keys.'
    #[builder(into, default)]
    #[serde(rename = "kmsKey")]
    pub r#kms_key: Box<Option<String>>,
}
