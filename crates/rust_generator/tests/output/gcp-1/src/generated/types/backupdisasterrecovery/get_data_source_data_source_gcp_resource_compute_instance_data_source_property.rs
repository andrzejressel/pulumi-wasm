#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetDataSourceDataSourceGcpResourceComputeInstanceDataSourceProperty {
    /// The description of the Compute Engine instance.
    #[builder(into)]
    #[serde(rename = "description")]
    pub r#description: Box<String>,
    /// The machine type of the instance.
    #[builder(into)]
    #[serde(rename = "machineType")]
    pub r#machine_type: Box<String>,
    /// Name of the compute instance backed up by the datasource.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// The total number of disks attached to the Instance.
    #[builder(into)]
    #[serde(rename = "totalDiskCount")]
    pub r#total_disk_count: Box<String>,
    /// The sum of all the disk sizes.
    #[builder(into)]
    #[serde(rename = "totalDiskSizeGb")]
    pub r#total_disk_size_gb: Box<String>,
}
