#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct WorkflowTemplatePlacementManagedClusterConfigSecondaryWorkerConfigDiskConfig {
    /// Size in GB of the boot disk (default is 500GB).
    #[builder(into, default)]
    #[serde(rename = "bootDiskSizeGb")]
    pub r#boot_disk_size_gb: Box<Option<i32>>,
    /// Type of the boot disk (default is "pd-standard"). Valid values: "pd-ssd" (Persistent Disk Solid State Drive) or "pd-standard" (Persistent Disk Hard Disk Drive).
    #[builder(into, default)]
    #[serde(rename = "bootDiskType")]
    pub r#boot_disk_type: Box<Option<String>>,
    /// Number of attached SSDs, from 0 to 4 (default is 0). If SSDs are not attached, the boot disk is used to store runtime logs and (https://hadoop.apache.org/docs/r1.2.1/hdfs_user_guide.html) data. If one or more SSDs are attached, this runtime bulk data is spread across them, and the boot disk contains only basic config and installed binaries.
    #[builder(into, default)]
    #[serde(rename = "numLocalSsds")]
    pub r#num_local_ssds: Box<Option<i32>>,
}
