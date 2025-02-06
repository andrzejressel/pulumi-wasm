#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct VirtualMachineStorageOsDisk {
    /// Specifies the caching requirements for the OS Disk. Possible values include `None`, `ReadOnly` and `ReadWrite`.
    #[builder(into, default)]
    #[serde(rename = "caching")]
    pub r#caching: Box<Option<String>>,
    /// Specifies how the OS Disk should be created. Possible values are `Attach` (managed disks only) and `FromImage`.
    #[builder(into)]
    #[serde(rename = "createOption")]
    pub r#create_option: Box<String>,
    /// Specifies the size of the OS Disk in gigabytes.
    #[builder(into, default)]
    #[serde(rename = "diskSizeGb")]
    pub r#disk_size_gb: Box<Option<i32>>,
    /// Specifies the Image URI in the format `publisherName:offer:skus:version`. This field can also specify the [VHD URI](https://docs.microsoft.com/azure/virtual-machines/linux/tutorial-custom-images) of a custom VM image to clone. When cloning a Custom (Unmanaged) Disk Image the `os_type` field must be set.
    #[builder(into, default)]
    #[serde(rename = "imageUri")]
    pub r#image_uri: Box<Option<String>>,
    /// Specifies the ID of an existing Managed Disk which should be attached as the OS Disk of this Virtual Machine. If this is set then the `create_option` must be set to `Attach`. Changing this forces a new resource to be created.
    #[builder(into, default)]
    #[serde(rename = "managedDiskId")]
    pub r#managed_disk_id: Box<Option<String>>,
    /// Specifies the type of Managed Disk which should be created. Possible values are `Standard_LRS`, `StandardSSD_LRS` or `Premium_LRS`.
    /// 
    /// The following properties apply when using Unmanaged Disks:
    #[builder(into, default)]
    #[serde(rename = "managedDiskType")]
    pub r#managed_disk_type: Box<Option<String>>,
    /// Specifies the name of the OS Disk.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// Specifies the Operating System on the OS Disk. Possible values are `Linux` and `Windows`.
    #[builder(into, default)]
    #[serde(rename = "osType")]
    pub r#os_type: Box<Option<String>>,
    /// Specifies the URI of the VHD file backing this Unmanaged OS Disk. Changing this forces a new resource to be created.
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
