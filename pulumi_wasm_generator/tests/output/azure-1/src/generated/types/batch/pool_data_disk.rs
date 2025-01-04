#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct PoolDataDisk {
    /// Values are: "none" - The caching mode for the disk is not enabled. "readOnly" - The caching mode for the disk is read only. "readWrite" - The caching mode for the disk is read and write. For information about the caching options see: <https://blogs.msdn.microsoft.com/windowsazurestorage/2012/06/27/exploring-windows-azure-drives-disks-and-images/>. Possible values are `None`, `ReadOnly` and `ReadWrite`. Defaults to `ReadOnly`.
    #[builder(into, default)]
    #[serde(rename = "caching")]
    pub r#caching: Box<Option<String>>,
    /// The initial disk size in GB when creating new data disk.
    #[builder(into)]
    #[serde(rename = "diskSizeGb")]
    pub r#disk_size_gb: Box<i32>,
    /// The lun is used to uniquely identify each data disk. If attaching multiple disks, each should have a distinct lun. The value must be between 0 and 63, inclusive.
    #[builder(into)]
    #[serde(rename = "lun")]
    pub r#lun: Box<i32>,
    /// The storage account type to be used for the data disk. Values are: Possible values are `Standard_LRS` - The data disk should use standard locally redundant storage. `Premium_LRS` - The data disk should use premium locally redundant storage. Defaults to `Standard_LRS`.
    #[builder(into, default)]
    #[serde(rename = "storageAccountType")]
    pub r#storage_account_type: Box<Option<String>>,
}
