#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ConnectorCapacityAutoscalingScaleOutPolicy {
    /// The CPU utilization percentage threshold at which you want connector scale out to be triggered.
    #[builder(into, default)]
    #[serde(rename = "cpuUtilizationPercentage")]
    pub r#cpu_utilization_percentage: Box<Option<i32>>,
}