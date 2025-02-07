#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct OrchestratedVirtualMachineScaleSetDataDisk {
    /// The type of Caching which should be used for this Data Disk. Possible values are None, ReadOnly and ReadWrite.
    #[builder(into)]
    #[serde(rename = "caching")]
    pub r#caching: Box<String>,
    /// The create option which should be used for this Data Disk. Possible values are Empty and FromImage. Defaults to `Empty`. (FromImage should only be used if the source image includes data disks).
    #[builder(into, default)]
    #[serde(rename = "createOption")]
    pub r#create_option: Box<Option<String>>,
    /// The ID of the Disk Encryption Set which should be used to encrypt the Data Disk. Changing this forces a new resource to be created.
    #[builder(into, default)]
    #[serde(rename = "diskEncryptionSetId")]
    pub r#disk_encryption_set_id: Box<Option<String>>,
    /// The size of the Data Disk which should be created. Required if `create_option` is specified as `Empty`.
    #[builder(into, default)]
    #[serde(rename = "diskSizeGb")]
    pub r#disk_size_gb: Box<Option<i32>>,
    /// The Logical Unit Number of the Data Disk, which must be unique within the Virtual Machine. Required if `create_option` is specified as `Empty`.
    #[builder(into, default)]
    #[serde(rename = "lun")]
    pub r#lun: Box<Option<i32>>,
    /// The Type of Storage Account which should back this Data Disk. Possible values include `Standard_LRS`, `StandardSSD_LRS`, `StandardSSD_ZRS`, `Premium_LRS`, `PremiumV2_LRS`, `Premium_ZRS` and `UltraSSD_LRS`.
    #[builder(into)]
    #[serde(rename = "storageAccountType")]
    pub r#storage_account_type: Box<String>,
    /// Specifies the Read-Write IOPS for this Data Disk. Only settable when `storage_account_type` is `PremiumV2_LRS` or `UltraSSD_LRS`.
    #[builder(into, default)]
    #[serde(rename = "ultraSsdDiskIopsReadWrite")]
    pub r#ultra_ssd_disk_iops_read_write: Box<Option<i32>>,
    /// Specifies the bandwidth in MB per second for this Data Disk. Only settable when `storage_account_type` is `PremiumV2_LRS` or `UltraSSD_LRS`.
    #[builder(into, default)]
    #[serde(rename = "ultraSsdDiskMbpsReadWrite")]
    pub r#ultra_ssd_disk_mbps_read_write: Box<Option<i32>>,
    /// Specifies if Write Accelerator is enabled on the Data Disk. Defaults to `false`.
    #[builder(into, default)]
    #[serde(rename = "writeAcceleratorEnabled")]
    pub r#write_accelerator_enabled: Box<Option<bool>>,
}
