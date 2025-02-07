#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetClusterNodePoolUpgradeSettingBlueGreenSettingStandardRolloutPolicy {
    /// Number of blue nodes to drain in a batch.
    #[builder(into)]
    #[serde(rename = "batchNodeCount")]
    pub r#batch_node_count: Box<i32>,
    /// Percentage of the blue pool nodes to drain in a batch.
    #[builder(into)]
    #[serde(rename = "batchPercentage")]
    pub r#batch_percentage: Box<f64>,
    /// Soak time after each batch gets drained.
    #[builder(into)]
    #[serde(rename = "batchSoakDuration")]
    pub r#batch_soak_duration: Box<String>,
}
