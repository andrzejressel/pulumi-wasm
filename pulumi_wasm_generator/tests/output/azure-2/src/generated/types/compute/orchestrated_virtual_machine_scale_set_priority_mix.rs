#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct OrchestratedVirtualMachineScaleSetPriorityMix {
    /// Specifies the base number of VMs of `Regular` priority that will be created before any VMs of priority `Spot` are created. Possible values are integers between `0` and `1000`. Defaults to `0`.
    #[builder(into, default)]
    #[serde(rename = "baseRegularCount")]
    pub r#base_regular_count: Box<Option<i32>>,
    /// Specifies the desired percentage of VM instances that are of `Regular` priority after the base count has been reached. Possible values are integers between `0` and `100`. Defaults to `0`.
    #[builder(into, default)]
    #[serde(rename = "regularPercentageAboveBase")]
    pub r#regular_percentage_above_base: Box<Option<i32>>,
}
