#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct WorkstationConfigPersistentDirectoryGcePd {
    /// Type of the disk to use. Defaults to `"pd-standard"`.
    #[builder(into, default)]
    #[serde(rename = "diskType")]
    pub r#disk_type: Box<Option<String>>,
    /// Type of file system that the disk should be formatted with. The workstation image must support this file system type. Must be empty if `sourceSnapshot` is set. Defaults to `ext4`.
    #[builder(into, default)]
    #[serde(rename = "fsType")]
    pub r#fs_type: Box<Option<String>>,
    /// Whether the persistent disk should be deleted when the workstation is deleted. Valid values are `DELETE` and `RETAIN`. Defaults to `DELETE`.
    /// Possible values are: `DELETE`, `RETAIN`.
    #[builder(into, default)]
    #[serde(rename = "reclaimPolicy")]
    pub r#reclaim_policy: Box<Option<String>>,
    /// The GB capacity of a persistent home directory for each workstation created with this configuration. Must be empty if `sourceSnapshot` is set.
    /// Valid values are `10`, `50`, `100`, `200`, `500`, or `1000`. Defaults to `200`. If less than `200` GB, the `diskType` must be `pd-balanced` or `pd-ssd`.
    #[builder(into, default)]
    #[serde(rename = "sizeGb")]
    pub r#size_gb: Box<Option<i32>>,
    /// Name of the snapshot to use as the source for the disk.
    /// Must be empty if `sourceImage` is set.
    /// Must be empty if `read_only` is false.
    /// Updating `source_snapshot` will update content in the ephemeral directory after the workstation is restarted.
    #[builder(into, default)]
    #[serde(rename = "sourceSnapshot")]
    pub r#source_snapshot: Box<Option<String>>,
}
