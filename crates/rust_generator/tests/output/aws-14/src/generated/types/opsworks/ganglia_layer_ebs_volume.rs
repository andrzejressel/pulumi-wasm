#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct GangliaLayerEbsVolume {
    #[builder(into, default)]
    #[serde(rename = "encrypted")]
    pub r#encrypted: Box<Option<bool>>,
    /// For PIOPS volumes, the IOPS per disk.
    #[builder(into, default)]
    #[serde(rename = "iops")]
    pub r#iops: Box<Option<i32>>,
    /// The path to mount the EBS volume on the layer's instances.
    #[builder(into)]
    #[serde(rename = "mountPoint")]
    pub r#mount_point: Box<String>,
    /// The number of disks to use for the EBS volume.
    #[builder(into)]
    #[serde(rename = "numberOfDisks")]
    pub r#number_of_disks: Box<i32>,
    /// The RAID level to use for the volume.
    #[builder(into, default)]
    #[serde(rename = "raidLevel")]
    pub r#raid_level: Box<Option<String>>,
    /// The size of the volume in gigabytes.
    #[builder(into)]
    #[serde(rename = "size")]
    pub r#size: Box<i32>,
    /// The type of volume to create. This may be `standard` (the default), `io1` or `gp2`.
    #[builder(into, default)]
    #[serde(rename = "type")]
    pub r#type_: Box<Option<String>>,
}
