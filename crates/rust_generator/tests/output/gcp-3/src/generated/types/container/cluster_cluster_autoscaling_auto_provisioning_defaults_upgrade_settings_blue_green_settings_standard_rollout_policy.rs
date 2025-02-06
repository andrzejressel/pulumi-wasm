#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ClusterClusterAutoscalingAutoProvisioningDefaultsUpgradeSettingsBlueGreenSettingsStandardRolloutPolicy {
    /// Number of blue nodes to drain in a batch. Only one of the batch_percentage or batch_node_count can be specified.
    #[builder(into, default)]
    #[serde(rename = "batchNodeCount")]
    pub r#batch_node_count: Box<Option<i32>>,
    /// Percentage of the bool pool nodes to drain in a batch. The range of this field should be (0.0, 1.0). Only one of the batch_percentage or batch_node_count can be specified.
    #[builder(into, default)]
    #[serde(rename = "batchPercentage")]
    pub r#batch_percentage: Box<Option<f64>>,
    /// Soak time after each batch gets drained. A duration in seconds with up to nine fractional digits, ending with 's'. Example: "3.5s".`.
    #[builder(into, default)]
    #[serde(rename = "batchSoakDuration")]
    pub r#batch_soak_duration: Box<Option<String>>,
}
