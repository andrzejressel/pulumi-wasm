#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct WorkstationConfigEphemeralDirectoryGcePd {
    /// Type of the disk to use. Defaults to `"pd-standard"`.
    #[builder(into, default)]
    #[serde(rename = "diskType")]
    pub r#disk_type: Box<Option<String>>,
    /// Whether the disk is read only. If true, the disk may be shared by multiple VMs and `sourceSnapshot` must be set.
    #[builder(into, default)]
    #[serde(rename = "readOnly")]
    pub r#read_only: Box<Option<bool>>,
    /// Name of the disk image to use as the source for the disk.
    /// Must be empty `sourceSnapshot` is set.
    /// Updating `sourceImage` will update content in the ephemeral directory after the workstation is restarted.
    #[builder(into, default)]
    #[serde(rename = "sourceImage")]
    pub r#source_image: Box<Option<String>>,
    /// Name of the snapshot to use as the source for the disk.
    /// Must be empty if `sourceImage` is set.
    /// Must be empty if `read_only` is false.
    /// Updating `source_snapshot` will update content in the ephemeral directory after the workstation is restarted.
    #[builder(into, default)]
    #[serde(rename = "sourceSnapshot")]
    pub r#source_snapshot: Box<Option<String>>,
}
