#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct RegionAutoscalerAutoscalingPolicyScaleInControl {
    /// A nested object resource.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "maxScaledInReplicas")]
    pub r#max_scaled_in_replicas: Box<Option<super::super::types::compute::RegionAutoscalerAutoscalingPolicyScaleInControlMaxScaledInReplicas>>,
    /// How long back autoscaling should look when computing recommendations
    /// to include directives regarding slower scale down, as described above.
    #[builder(into, default)]
    #[serde(rename = "timeWindowSec")]
    pub r#time_window_sec: Box<Option<i32>>,
}
