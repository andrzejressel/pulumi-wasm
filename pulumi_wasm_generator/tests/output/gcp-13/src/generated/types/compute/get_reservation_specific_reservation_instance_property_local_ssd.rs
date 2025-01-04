#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetReservationSpecificReservationInstancePropertyLocalSsd {
    /// The size of the disk in base-2 GB.
    #[builder(into)]
    #[serde(rename = "diskSizeGb")]
    pub r#disk_size_gb: Box<i32>,
    /// The disk interface to use for attaching this disk. Default value: "SCSI" Possible values: ["SCSI", "NVME"]
    #[builder(into)]
    #[serde(rename = "interface")]
    pub r#interface: Box<String>,
}
