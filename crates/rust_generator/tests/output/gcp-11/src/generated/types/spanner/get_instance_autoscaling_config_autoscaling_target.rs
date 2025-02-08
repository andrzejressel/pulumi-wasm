#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct GetInstanceAutoscalingConfigAutoscalingTarget {
    /// Specifies the target high priority cpu utilization percentage that the autoscaler
    /// should be trying to achieve for the instance.
    /// This number is on a scale from 0 (no utilization) to 100 (full utilization)..
    #[builder(into)]
    #[serde(rename = "highPriorityCpuUtilizationPercent")]
    pub r#high_priority_cpu_utilization_percent: Box<i32>,
    /// Specifies the target storage utilization percentage that the autoscaler
    /// should be trying to achieve for the instance.
    /// This number is on a scale from 0 (no utilization) to 100 (full utilization).
    #[builder(into)]
    #[serde(rename = "storageUtilizationPercent")]
    pub r#storage_utilization_percent: Box<i32>,
}
