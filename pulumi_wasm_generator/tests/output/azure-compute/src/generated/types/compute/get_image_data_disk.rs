#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetImageDataDisk {
    /// the URI in Azure storage of the blob used to create the image.
    #[builder(into)]
    #[serde(rename = "blobUri")]
    pub r#blob_uri: Box<String>,
    /// the caching mode for the Data Disk, such as `ReadWrite`, `ReadOnly`, or `None`.
    #[builder(into)]
    #[serde(rename = "caching")]
    pub r#caching: Box<String>,
    /// the logical unit number of the data disk.
    #[builder(into)]
    #[serde(rename = "lun")]
    pub r#lun: Box<i32>,
    /// the ID of the Managed Disk used as the Data Disk Image.
    #[builder(into)]
    #[serde(rename = "managedDiskId")]
    pub r#managed_disk_id: Box<String>,
    /// the size of this Data Disk in GB.
    #[builder(into)]
    #[serde(rename = "sizeGb")]
    pub r#size_gb: Box<i32>,
}