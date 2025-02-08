#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct VmwareReplicatedVmManagedDisk {
    /// The ID of the disk to be replicated.
    #[builder(into)]
    #[serde(rename = "diskId")]
    pub r#disk_id: Box<String>,
    /// The ID of the storage account that should be used for logging during replication.
    #[builder(into, default)]
    #[serde(rename = "logStorageAccountId")]
    pub r#log_storage_account_id: Box<Option<String>>,
    /// The ID of the Disk Encryption Set that should be used for the disks when a failover is done.
    #[builder(into, default)]
    #[serde(rename = "targetDiskEncryptionSetId")]
    pub r#target_disk_encryption_set_id: Box<Option<String>>,
    /// The disk type of the disk to be created when a failover is done. Possible values are `Premium_LRS`, `Standard_LRS` and `StandardSSD_LRS`.
    #[builder(into)]
    #[serde(rename = "targetDiskType")]
    pub r#target_disk_type: Box<String>,
}
