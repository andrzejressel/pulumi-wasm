#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ClusterCoreInstanceGroupEbsConfig {
    /// Number of I/O operations per second (IOPS) that the volume supports.
    #[builder(into, default)]
    #[serde(rename = "iops")]
    pub r#iops: Box<Option<i32>>,
    /// Volume size, in gibibytes (GiB).
    #[builder(into)]
    #[serde(rename = "size")]
    pub r#size: Box<i32>,
    /// The throughput, in mebibyte per second (MiB/s).
    #[builder(into, default)]
    #[serde(rename = "throughput")]
    pub r#throughput: Box<Option<i32>>,
    /// Volume type. Valid options are `gp3`, `gp2`, `io1`, `io2`, `standard`, `st1` and `sc1`. See [EBS Volume Types](https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/EBSVolumeTypes.html).
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: Box<String>,
    /// Number of EBS volumes with this configuration to attach to each EC2 instance in the instance group (default is 1).
    #[builder(into, default)]
    #[serde(rename = "volumesPerInstance")]
    pub r#volumes_per_instance: Box<Option<i32>>,
}
