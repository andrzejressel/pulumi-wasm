#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct AmiCopyEbsBlockDevice {
    /// Boolean controlling whether the EBS volumes created to
    /// support each created instance will be deleted once that instance is terminated.
    #[builder(into, default)]
    #[serde(rename = "deleteOnTermination")]
    pub r#delete_on_termination: Box<Option<bool>>,
    /// Path at which the device is exposed to created instances.
    #[builder(into, default)]
    #[serde(rename = "deviceName")]
    pub r#device_name: Box<Option<String>>,
    /// Boolean controlling whether the created EBS volumes will be encrypted. Can't be used with `snapshot_id`.
    #[builder(into, default)]
    #[serde(rename = "encrypted")]
    pub r#encrypted: Box<Option<bool>>,
    /// Number of I/O operations per second the
    /// created volumes will support.
    #[builder(into, default)]
    #[serde(rename = "iops")]
    pub r#iops: Box<Option<i32>>,
    /// ARN of the Outpost on which the snapshot is stored.
    /// 
    /// > **Note:** You can specify `encrypted` or `snapshot_id` but not both.
    #[builder(into, default)]
    #[serde(rename = "outpostArn")]
    pub r#outpost_arn: Box<Option<String>>,
    /// ID of an EBS snapshot that will be used to initialize the created
    /// EBS volumes. If set, the `volume_size` attribute must be at least as large as the referenced
    /// snapshot.
    #[builder(into, default)]
    #[serde(rename = "snapshotId")]
    pub r#snapshot_id: Box<Option<String>>,
    /// Throughput that the EBS volume supports, in MiB/s. Only valid for `volume_type` of `gp3`.
    #[builder(into, default)]
    #[serde(rename = "throughput")]
    pub r#throughput: Box<Option<i32>>,
    /// Size of created volumes in GiB.
    /// If `snapshot_id` is set and `volume_size` is omitted then the volume will have the same size
    /// as the selected snapshot.
    #[builder(into, default)]
    #[serde(rename = "volumeSize")]
    pub r#volume_size: Box<Option<i32>>,
    /// Type of EBS volume to create. Can be `standard`, `gp2`, `gp3`, `io1`, `io2`, `sc1` or `st1` (Default: `standard`).
    #[builder(into, default)]
    #[serde(rename = "volumeType")]
    pub r#volume_type: Box<Option<String>>,
}
