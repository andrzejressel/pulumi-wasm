#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct ScaleSetStorageProfileOsDisk {
    /// Specifies the caching requirements. Possible values include: `None` (default), `ReadOnly`, `ReadWrite`.
    #[builder(into, default)]
    #[serde(rename = "caching")]
    pub r#caching: Box<Option<String>>,
    /// Specifies how the virtual machine should be created. The only possible option is `FromImage`.
    #[builder(into)]
    #[serde(rename = "createOption")]
    pub r#create_option: Box<String>,
    /// Specifies the blob URI for user image. A virtual machine scale set creates an os disk in the same container as the user image.
    /// Updating the osDisk image causes the existing disk to be deleted and a new one created with the new image. If the VM scale set is in Manual upgrade mode then the virtual machines are not updated until they have manualUpgrade applied to them.
    /// When setting this field `os_type` needs to be specified. Cannot be used when `vhd_containers`, `managed_disk_type` or `storage_profile_image_reference` are specified.
    #[builder(into, default)]
    #[serde(rename = "image")]
    pub r#image: Box<Option<String>>,
    /// Specifies the type of managed disk to create. Value you must be either `Standard_LRS`, `StandardSSD_LRS` or `Premium_LRS`. Cannot be used when `vhd_containers` or `image` is specified.
    #[builder(into, default)]
    #[serde(rename = "managedDiskType")]
    pub r#managed_disk_type: Box<Option<String>>,
    /// Specifies the disk name. Must be specified when using unmanaged disk ('managed_disk_type' property not set).
    #[builder(into, default)]
    #[serde(rename = "name")]
    pub r#name: Box<Option<String>>,
    /// Specifies the operating system Type, valid values are windows, Linux.
    #[builder(into, default)]
    #[serde(rename = "osType")]
    pub r#os_type: Box<Option<String>>,
    /// Specifies the VHD URI. Cannot be used when `image` or `managed_disk_type` is specified.
    #[builder(into, default)]
    #[serde(rename = "vhdContainers")]
    pub r#vhd_containers: Box<Option<Vec<String>>>,
}
