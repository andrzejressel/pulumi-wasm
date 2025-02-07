#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ImageDataDisk {
    /// Specifies the URI in Azure storage of the blob that you want to use to create the image.
    #[builder(into, default)]
    #[serde(rename = "blobUri")]
    pub r#blob_uri: Box<Option<String>>,
    /// Specifies the caching mode as `ReadWrite`, `ReadOnly`, or `None`. Defaults to `None`.
    #[builder(into, default)]
    #[serde(rename = "caching")]
    pub r#caching: Box<Option<String>>,
    /// The ID of the Disk Encryption Set which should be used to encrypt this disk. Changing this forces a new resource to be created.
    #[builder(into, default)]
    #[serde(rename = "diskEncryptionSetId")]
    pub r#disk_encryption_set_id: Box<Option<String>>,
    /// Specifies the logical unit number of the data disk.
    #[builder(into, default)]
    #[serde(rename = "lun")]
    pub r#lun: Box<Option<i32>>,
    /// Specifies the ID of the managed disk resource that you want to use to create the image. Changing this forces a new resource to be created.
    #[builder(into, default)]
    #[serde(rename = "managedDiskId")]
    pub r#managed_disk_id: Box<Option<String>>,
    /// Specifies the size of the image to be created. The target size can't be smaller than the source size.
    #[builder(into, default)]
    #[serde(rename = "sizeGb")]
    pub r#size_gb: Box<Option<i32>>,
    /// The type of Storage Disk to use. Possible values are `Premium_LRS`, `PremiumV2_LRS`, `Premium_ZRS`, `Standard_LRS`, `StandardSSD_LRS`, `StandardSSD_ZRS` and `UltraSSD_LRS`. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "storageType")]
    pub r#storage_type: Box<String>,
}
