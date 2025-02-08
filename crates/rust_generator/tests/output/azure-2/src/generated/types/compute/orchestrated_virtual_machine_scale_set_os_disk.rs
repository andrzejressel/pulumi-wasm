#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct OrchestratedVirtualMachineScaleSetOsDisk {
    /// The Type of Caching which should be used for the Internal OS Disk. Possible values are `None`, `ReadOnly` and `ReadWrite`.
    #[builder(into)]
    #[serde(rename = "caching")]
    pub r#caching: Box<String>,
    /// A `diff_disk_settings` block as defined above. Changing this forces a new resource to be created.
    #[builder(into, default)]
    #[serde(rename = "diffDiskSettings")]
    pub r#diff_disk_settings: Box<Option<super::super::types::compute::OrchestratedVirtualMachineScaleSetOsDiskDiffDiskSettings>>,
    /// The ID of the Disk Encryption Set which should be used to encrypt this OS Disk. Changing this forces a new resource to be created.
    /// 
    /// > **Note:** Disk Encryption Sets are in Public Preview in a limited set of regions
    #[builder(into, default)]
    #[serde(rename = "diskEncryptionSetId")]
    pub r#disk_encryption_set_id: Box<Option<String>>,
    /// The Size of the Internal OS Disk in GB, if you wish to vary from the size used in the image this Virtual Machine Scale Set is sourced from.
    #[builder(into, default)]
    #[serde(rename = "diskSizeGb")]
    pub r#disk_size_gb: Box<Option<i32>>,
    /// The Type of Storage Account which should back this the Internal OS Disk. Possible values include `Standard_LRS`, `StandardSSD_LRS`, `StandardSSD_ZRS`, `Premium_LRS` and `Premium_ZRS`. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "storageAccountType")]
    pub r#storage_account_type: Box<String>,
    /// Specifies if Write Accelerator is enabled on the OS Disk. Defaults to `false`.
    #[builder(into, default)]
    #[serde(rename = "writeAcceleratorEnabled")]
    pub r#write_accelerator_enabled: Box<Option<bool>>,
}
