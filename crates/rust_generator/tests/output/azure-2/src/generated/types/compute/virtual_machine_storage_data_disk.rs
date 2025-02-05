#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct VirtualMachineStorageDataDisk {
    /// Specifies the caching requirements for the Data Disk. Possible values include `None`, `ReadOnly` and `ReadWrite`.
    #[builder(into, default)]
    #[serde(rename = "caching")]
    pub r#caching: Box<Option<String>>,
    /// Specifies how the data disk should be created. Possible values are `Attach`, `FromImage` and `Empty`.
    /// 
    /// > **NOTE:** If using an image that does not have data to be written to the Data Disk, use `Empty` as the create option in order to create the desired disk without any data.
    #[builder(into)]
    #[serde(rename = "createOption")]
    pub r#create_option: Box<String>,
    /// Specifies the size of the data disk in gigabytes.
    #[builder(into, default)]
    #[serde(rename = "diskSizeGb")]
    pub r#disk_size_gb: Box<Option<i32>>,
    /// Specifies the logical unit number of the data disk. This needs to be unique within all the Data Disks on the Virtual Machine.
    #[builder(into)]
    #[serde(rename = "lun")]
    pub r#lun: Box<i32>,
    /// Specifies the ID of an Existing Managed Disk which should be attached to this Virtual Machine. When this field is set `create_option` must be set to `Attach`.
    /// 
    /// The following properties apply when using Unmanaged Disks:
    #[builder(into, default)]
    #[serde(rename = "managedDiskId")]
    pub r#managed_disk_id: Box<Option<String>>,
    /// Specifies the type of managed disk to create. Possible values are either `Standard_LRS`, `StandardSSD_LRS`, `Premium_LRS` or `UltraSSD_LRS`.
    /// 
    /// > **Note:** `managed_disk_type` of type `UltraSSD_LRS` is currently in preview and are not available to subscriptions that have not [requested](https://aka.ms/UltraSSDPreviewSignUp) onboarding to `Azure Ultra Disk Storage` preview. `Azure Ultra Disk Storage` is only available in `East US 2`, `North Europe`, and `Southeast Asia` regions. For more information see the `Azure Ultra Disk Storage` [product documentation](https://docs.microsoft.com/azure/virtual-machines/windows/disks-enable-ultra-ssd), [product blog](https://azure.microsoft.com/en-us/blog/announcing-the-general-availability-of-azure-ultra-disk-storage/) and [FAQ](https://docs.microsoft.com/azure/virtual-machines/windows/faq-for-disks#ultra-disks). You must also set `additional_capabilities.ultra_ssd_enabled` to `true`.
    #[builder(into, default)]
    #[serde(rename = "managedDiskType")]
    pub r#managed_disk_type: Box<Option<String>>,
    /// The name of the Data Disk.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// Specifies the URI of the VHD file backing this Unmanaged Data Disk.
    #[builder(into, default)]
    #[serde(rename = "vhdUri")]
    pub r#vhd_uri: Box<Option<String>>,
    /// Specifies if Write Accelerator is enabled on the disk. This can only be enabled on `Premium_LRS` managed disks with no caching and [M-Series VMs](https://docs.microsoft.com/azure/virtual-machines/workloads/sap/how-to-enable-write-accelerator). Defaults to `false`.
    /// 
    /// The following properties apply when using Managed Disks:
    #[builder(into, default)]
    #[serde(rename = "writeAcceleratorEnabled")]
    pub r#write_accelerator_enabled: Box<Option<bool>>,
}
