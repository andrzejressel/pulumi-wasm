#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ReplicatedVmManagedDisk {
    /// Id of disk that should be replicated. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "diskId")]
    pub r#disk_id: Box<String>,
    /// Storage account that should be used for caching. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "stagingStorageAccountId")]
    pub r#staging_storage_account_id: Box<String>,
    /// A `target_disk_encryption` block as defined below.
    #[builder(into, default)]
    #[serde(rename = "targetDiskEncryption")]
    pub r#target_disk_encryption: Box<Option<super::super::types::siterecovery::ReplicatedVmManagedDiskTargetDiskEncryption>>,
    /// The Disk Encryption Set that the Managed Disk will be associated with. Changing this forces a new resource to be created.
    /// 
    /// > **NOTE:** Creating replicated vm with `target_disk_encryption_set_id` wil take more time (up to 5 hours), please extend the `timeout` for `create`.
    #[builder(into, default)]
    #[serde(rename = "targetDiskEncryptionSetId")]
    pub r#target_disk_encryption_set_id: Box<Option<String>>,
    /// What type should the disk be when a failover is done. Possible values are `Standard_LRS`, `Premium_LRS`, `StandardSSD_LRS` and `UltraSSD_LRS`. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "targetDiskType")]
    pub r#target_disk_type: Box<String>,
    /// What type should the disk be that holds the replication data. Possible values are `Standard_LRS`, `Premium_LRS`, `StandardSSD_LRS` and `UltraSSD_LRS`. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "targetReplicaDiskType")]
    pub r#target_replica_disk_type: Box<String>,
    /// Resource group disk should belong to when a failover is done. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "targetResourceGroupId")]
    pub r#target_resource_group_id: Box<String>,
}
