#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetClusterClusterAutoscalingAutoProvisioningDefaultUpgradeSettingBlueGreenSettingStandardRolloutPolicy {
    /// Number of blue nodes to drain in a batch.
    #[builder(into)]
    #[serde(rename = "batchNodeCount")]
    pub r#batch_node_count: Box<i32>,
    /// Percentage of the bool pool nodes to drain in a batch. The range of this field should be (0.0, 1.0].
    #[builder(into)]
    #[serde(rename = "batchPercentage")]
    pub r#batch_percentage: Box<f64>,
    /// Soak time after each batch gets drained.
    /// 
    /// A duration in seconds with up to nine fractional digits, ending with 's'. Example: "3.5s".
    #[builder(into)]
    #[serde(rename = "batchSoakDuration")]
    pub r#batch_soak_duration: Box<String>,
}
