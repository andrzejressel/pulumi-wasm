#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct PipeTargetParametersEcsTaskParametersOverridesEphemeralStorage {
    /// The total amount, in GiB, of ephemeral storage to set for the task. The minimum supported value is 21 GiB and the maximum supported value is 200 GiB.
    #[builder(into)]
    #[serde(rename = "sizeInGib")]
    pub r#size_in_gib: Box<i32>,
}