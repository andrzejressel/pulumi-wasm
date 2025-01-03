#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct InstanceGroupEbsConfig {
    /// The number of I/O operations per second (IOPS) that the volume supports.
    #[builder(into, default)]
    #[serde(rename = "iops")]
    pub r#iops: Box<Option<i32>>,
    /// The volume size, in gibibytes (GiB). This can be a number from 1 - 1024. If the volume type is EBS-optimized, the minimum value is 10.
    #[builder(into)]
    #[serde(rename = "size")]
    pub r#size: Box<i32>,
    /// The volume type. Valid options are 'gp2', 'io1' and 'standard'.
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: Box<String>,
    /// The number of EBS Volumes to attach per instance.
    #[builder(into, default)]
    #[serde(rename = "volumesPerInstance")]
    pub r#volumes_per_instance: Box<Option<i32>>,
}
