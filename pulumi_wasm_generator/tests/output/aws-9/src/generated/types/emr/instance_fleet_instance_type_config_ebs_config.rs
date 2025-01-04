#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct InstanceFleetInstanceTypeConfigEbsConfig {
    /// The number of I/O operations per second (IOPS) that the volume supports
    #[builder(into, default)]
    #[serde(rename = "iops")]
    pub r#iops: Box<Option<i32>>,
    /// The volume size, in gibibytes (GiB).
    #[builder(into)]
    #[serde(rename = "size")]
    pub r#size: Box<i32>,
    /// The volume type. Valid options are `gp2`, `io1`, `standard` and `st1`. See [EBS Volume Types](https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/EBSVolumeTypes.html).
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: Box<String>,
    /// The number of EBS volumes with this configuration to attach to each EC2 instance in the instance group (default is 1)
    #[builder(into, default)]
    #[serde(rename = "volumesPerInstance")]
    pub r#volumes_per_instance: Box<Option<i32>>,
}
