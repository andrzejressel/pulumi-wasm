#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct InstancePerformanceConfigFixedIops {
    /// The number of IOPS to provision for the instance.
    /// max_iops must be in multiple of 1000.
    #[builder(into, default)]
    #[serde(rename = "maxIops")]
    pub r#max_iops: Box<Option<i32>>,
}
