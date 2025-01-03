#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ClusterClusterConfigMasterConfigDiskConfig {
    /// Size of the primary disk attached to each node, specified
    /// in GB. The primary disk contains the boot volume and system libraries, and the
    /// smallest allowed disk size is 10GB. GCP will default to a predetermined
    /// computed value if not set (currently 500GB). Note: If SSDs are not
    /// attached, it also contains the HDFS data blocks and Hadoop working directories.
    #[builder(into, default)]
    #[serde(rename = "bootDiskSizeGb")]
    pub r#boot_disk_size_gb: Box<Option<i32>>,
    /// The disk type of the primary disk attached to each node.
    /// One of `"pd-ssd"` or `"pd-standard"`. Defaults to `"pd-standard"`.
    #[builder(into, default)]
    #[serde(rename = "bootDiskType")]
    pub r#boot_disk_type: Box<Option<String>>,
    /// Optional. Interface type of local SSDs (default is "scsi").
    /// Valid values: "scsi" (Small Computer System Interface), "nvme" (Non-Volatile
    /// Memory Express). See
    /// [local SSD performance](https://cloud.google.com/compute/docs/disks/local-ssd#performance).
    #[builder(into, default)]
    #[serde(rename = "localSsdInterface")]
    pub r#local_ssd_interface: Box<Option<String>>,
    /// The amount of local SSD disks that will be
    /// attached to each master cluster node. Defaults to 0.
    #[builder(into, default)]
    #[serde(rename = "numLocalSsds")]
    pub r#num_local_ssds: Box<Option<i32>>,
}
