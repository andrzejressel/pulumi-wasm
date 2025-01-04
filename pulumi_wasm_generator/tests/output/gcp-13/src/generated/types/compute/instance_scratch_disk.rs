#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct InstanceScratchDisk {
    /// Name with which the attached disk is accessible under /dev/disk/by-id/
    #[builder(into, default)]
    #[serde(rename = "deviceName")]
    pub r#device_name: Box<Option<String>>,
    /// The disk interface to use for attaching this disk; either SCSI or NVME.
    #[builder(into)]
    #[serde(rename = "interface")]
    pub r#interface: Box<String>,
    /// The size of the disk in gigabytes. One of 375 or 3000.
    #[builder(into, default)]
    #[serde(rename = "size")]
    pub r#size: Box<Option<i32>>,
}
