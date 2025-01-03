#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct AutoscalerAutoscalingPolicy {
    /// The number of seconds that the autoscaler should wait before it
    /// starts collecting information from a new instance. This prevents
    /// the autoscaler from collecting information when the instance is
    /// initializing, during which the collected usage would not be
    /// reliable. The default time autoscaler waits is 60 seconds.
    /// Virtual machine initialization times might vary because of
    /// numerous factors. We recommend that you test how long an
    /// instance may take to initialize. To do this, create an instance
    /// and time the startup process.
    #[builder(into, default)]
    #[serde(rename = "cooldownPeriod")]
    pub r#cooldown_period: Box<Option<i32>>,
    /// Defines the CPU utilization policy that allows the autoscaler to
    /// scale based on the average CPU utilization of a managed instance
    /// group.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "cpuUtilization")]
    pub r#cpu_utilization: Box<Option<super::super::types::compute::AutoscalerAutoscalingPolicyCpuUtilization>>,
    /// Configuration parameters of autoscaling based on a load balancer.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "loadBalancingUtilization")]
    pub r#load_balancing_utilization: Box<Option<super::super::types::compute::AutoscalerAutoscalingPolicyLoadBalancingUtilization>>,
    /// The maximum number of instances that the autoscaler can scale up
    /// to. This is required when creating or updating an autoscaler. The
    /// maximum number of replicas should not be lower than minimal number
    /// of replicas.
    #[builder(into)]
    #[serde(rename = "maxReplicas")]
    pub r#max_replicas: Box<i32>,
    /// Configuration parameters of autoscaling based on a custom metric.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "metrics")]
    pub r#metrics: Box<Option<Vec<super::super::types::compute::AutoscalerAutoscalingPolicyMetric>>>,
    /// The minimum number of replicas that the autoscaler can scale down
    /// to. This cannot be less than 0. If not provided, autoscaler will
    /// choose a default value depending on maximum number of instances
    /// allowed.
    #[builder(into)]
    #[serde(rename = "minReplicas")]
    pub r#min_replicas: Box<i32>,
    /// Defines operating mode for this policy.
    #[builder(into, default)]
    #[serde(rename = "mode")]
    pub r#mode: Box<Option<String>>,
    /// Defines scale down controls to reduce the risk of response latency
    /// and outages due to abrupt scale-in events
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "scaleDownControl")]
    pub r#scale_down_control: Box<Option<super::super::types::compute::AutoscalerAutoscalingPolicyScaleDownControl>>,
    /// Defines scale in controls to reduce the risk of response latency
    /// and outages due to abrupt scale-in events
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "scaleInControl")]
    pub r#scale_in_control: Box<Option<super::super::types::compute::AutoscalerAutoscalingPolicyScaleInControl>>,
    /// Scaling schedules defined for an autoscaler. Multiple schedules can be set on an autoscaler and they can overlap.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "scalingSchedules")]
    pub r#scaling_schedules: Box<Option<Vec<super::super::types::compute::AutoscalerAutoscalingPolicyScalingSchedule>>>,
}
