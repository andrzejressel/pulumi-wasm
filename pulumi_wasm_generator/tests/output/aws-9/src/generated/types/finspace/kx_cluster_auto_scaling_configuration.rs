#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct KxClusterAutoScalingConfiguration {
    /// Metric your cluster will track in order to scale in and out. For example, CPU_UTILIZATION_PERCENTAGE is the average CPU usage across all nodes in a cluster.
    #[builder(into)]
    #[serde(rename = "autoScalingMetric")]
    pub r#auto_scaling_metric: Box<String>,
    /// Highest number of nodes to scale. Cannot be greater than 5
    #[builder(into)]
    #[serde(rename = "maxNodeCount")]
    pub r#max_node_count: Box<i32>,
    /// Desired value of chosen `auto_scaling_metric`. When metric drops below this value, cluster will scale in. When metric goes above this value, cluster will scale out. Can be set between 0 and 100 percent.
    #[builder(into)]
    #[serde(rename = "metricTarget")]
    pub r#metric_target: Box<f64>,
    /// Lowest number of nodes to scale. Must be at least 1 and less than the `max_node_count`. If nodes in cluster belong to multiple availability zones, then `min_node_count` must be at least 3.
    #[builder(into)]
    #[serde(rename = "minNodeCount")]
    pub r#min_node_count: Box<i32>,
    /// Duration in seconds that FinSpace will wait after a scale in event before initiating another scaling event.
    #[builder(into)]
    #[serde(rename = "scaleInCooldownSeconds")]
    pub r#scale_in_cooldown_seconds: Box<f64>,
    /// Duration in seconds that FinSpace will wait after a scale out event before initiating another scaling event.
    #[builder(into)]
    #[serde(rename = "scaleOutCooldownSeconds")]
    pub r#scale_out_cooldown_seconds: Box<f64>,
}
