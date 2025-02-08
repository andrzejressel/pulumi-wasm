#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct GetImageOsDisk {
    /// the URI in Azure storage of the blob used to create the image.
    #[builder(into)]
    #[serde(rename = "blobUri")]
    pub r#blob_uri: Box<String>,
    /// the caching mode for the Data Disk, such as `ReadWrite`, `ReadOnly`, or `None`.
    #[builder(into)]
    #[serde(rename = "caching")]
    pub r#caching: Box<String>,
    /// the ID of the Managed Disk used as the Data Disk Image.
    #[builder(into)]
    #[serde(rename = "managedDiskId")]
    pub r#managed_disk_id: Box<String>,
    /// the State of the OS used in the Image, such as `Generalized`.
    #[builder(into)]
    #[serde(rename = "osState")]
    pub r#os_state: Box<String>,
    /// the type of Operating System used on the OS Disk. such as `Linux` or `Windows`.
    #[builder(into)]
    #[serde(rename = "osType")]
    pub r#os_type: Box<String>,
    /// the size of this Data Disk in GB.
    #[builder(into)]
    #[serde(rename = "sizeGb")]
    pub r#size_gb: Box<i32>,
}
