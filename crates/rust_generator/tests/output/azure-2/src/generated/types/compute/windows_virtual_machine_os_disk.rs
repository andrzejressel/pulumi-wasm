#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct WindowsVirtualMachineOsDisk {
    /// The Type of Caching which should be used for the Internal OS Disk. Possible values are `None`, `ReadOnly` and `ReadWrite`.
    #[builder(into)]
    #[serde(rename = "caching")]
    pub r#caching: Box<String>,
    /// A `diff_disk_settings` block as defined above. Changing this forces a new resource to be created.
    /// 
    /// > **NOTE:** `diff_disk_settings` can only be set when `caching` is set to `ReadOnly`. More information can be found [here](https://docs.microsoft.com/azure/virtual-machines/ephemeral-os-disks-deploy#vm-template-deployment)
    #[builder(into, default)]
    #[serde(rename = "diffDiskSettings")]
    pub r#diff_disk_settings: Box<Option<super::super::types::compute::WindowsVirtualMachineOsDiskDiffDiskSettings>>,
    /// The ID of the Disk Encryption Set which should be used to Encrypt this OS Disk. Conflicts with `secure_vm_disk_encryption_set_id`.
    /// 
    /// > **NOTE:** The Disk Encryption Set must have the `Reader` Role Assignment scoped on the Key Vault - in addition to an Access Policy to the Key Vault
    #[builder(into, default)]
    #[serde(rename = "diskEncryptionSetId")]
    pub r#disk_encryption_set_id: Box<Option<String>>,
    /// The Size of the Internal OS Disk in GB, if you wish to vary from the size used in the image this Virtual Machine is sourced from.
    /// 
    /// > **NOTE:** If specified this must be equal to or larger than the size of the Image the Virtual Machine is based on. When creating a larger disk than exists in the image you'll need to repartition the disk to use the remaining space.
    #[builder(into, default)]
    #[serde(rename = "diskSizeGb")]
    pub r#disk_size_gb: Box<Option<i32>>,
    /// The name which should be used for the Internal OS Disk. Changing this forces a new resource to be created.
    #[builder(into, default)]
    #[serde(rename = "name")]
    pub r#name: Box<Option<String>>,
    /// The ID of the Disk Encryption Set which should be used to Encrypt this OS Disk when the Virtual Machine is a Confidential VM. Conflicts with `disk_encryption_set_id`. Changing this forces a new resource to be created.
    /// 
    /// > **NOTE:** `secure_vm_disk_encryption_set_id` can only be specified when `security_encryption_type` is set to `DiskWithVMGuestState`.
    #[builder(into, default)]
    #[serde(rename = "secureVmDiskEncryptionSetId")]
    pub r#secure_vm_disk_encryption_set_id: Box<Option<String>>,
    /// Encryption Type when the Virtual Machine is a Confidential VM. Possible values are `VMGuestStateOnly` and `DiskWithVMGuestState`. Changing this forces a new resource to be created.
    /// 
    /// > **NOTE:** `vtpm_enabled` must be set to `true` when `security_encryption_type` is specified.
    /// 
    /// > **NOTE:** `encryption_at_host_enabled` cannot be set to `true` when `security_encryption_type` is set to `DiskWithVMGuestState`.
    #[builder(into, default)]
    #[serde(rename = "securityEncryptionType")]
    pub r#security_encryption_type: Box<Option<String>>,
    /// The Type of Storage Account which should back this the Internal OS Disk. Possible values are `Standard_LRS`, `StandardSSD_LRS`, `Premium_LRS`, `StandardSSD_ZRS` and `Premium_ZRS`. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "storageAccountType")]
    pub r#storage_account_type: Box<String>,
    /// Should Write Accelerator be Enabled for this OS Disk? Defaults to `false`.
    /// 
    /// > **NOTE:** This requires that the `storage_account_type` is set to `Premium_LRS` and that `caching` is set to `None`.
    #[builder(into, default)]
    #[serde(rename = "writeAcceleratorEnabled")]
    pub r#write_accelerator_enabled: Box<Option<bool>>,
}
