#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct AutoscalerAutoscalingPolicyCpuUtilization {
    /// Indicates whether predictive autoscaling based on CPU metric is enabled. Valid values are:
    /// - NONE (default). No predictive method is used. The autoscaler scales the group to meet current demand based on real-time metrics.
    /// - OPTIMIZE_AVAILABILITY. Predictive autoscaling improves availability by monitoring daily and weekly load patterns and scaling out ahead of anticipated demand.
    #[builder(into, default)]
    #[serde(rename = "predictiveMethod")]
    pub r#predictive_method: Box<Option<String>>,
    /// The target CPU utilization that the autoscaler should maintain.
    /// Must be a float value in the range (0, 1]. If not specified, the
    /// default is 0.6.
    /// If the CPU level is below the target utilization, the autoscaler
    /// scales down the number of instances until it reaches the minimum
    /// number of instances you specified or until the average CPU of
    /// your instances reaches the target utilization.
    /// If the average CPU is above the target utilization, the autoscaler
    /// scales up until it reaches the maximum number of instances you
    /// specified or until the average utilization reaches the target
    /// utilization.
    #[builder(into)]
    #[serde(rename = "target")]
    pub r#target: Box<f64>,
}
