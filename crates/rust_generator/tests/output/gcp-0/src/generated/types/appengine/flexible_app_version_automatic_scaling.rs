#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct FlexibleAppVersionAutomaticScaling {
    /// The time period that the Autoscaler should wait before it starts collecting information from a new instance.
    /// This prevents the autoscaler from collecting information when the instance is initializing,
    /// during which the collected usage would not be reliable. Default: 120s
    #[builder(into, default)]
    #[serde(rename = "coolDownPeriod")]
    pub r#cool_down_period: Box<Option<String>>,
    /// Target scaling by CPU usage.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "cpuUtilization")]
    pub r#cpu_utilization: Box<super::super::types::appengine::FlexibleAppVersionAutomaticScalingCpuUtilization>,
    /// Target scaling by disk usage.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "diskUtilization")]
    pub r#disk_utilization: Box<Option<super::super::types::appengine::FlexibleAppVersionAutomaticScalingDiskUtilization>>,
    /// Number of concurrent requests an automatic scaling instance can accept before the scheduler spawns a new instance.
    /// Defaults to a runtime-specific value.
    #[builder(into, default)]
    #[serde(rename = "maxConcurrentRequests")]
    pub r#max_concurrent_requests: Box<Option<i32>>,
    /// Maximum number of idle instances that should be maintained for this version.
    #[builder(into, default)]
    #[serde(rename = "maxIdleInstances")]
    pub r#max_idle_instances: Box<Option<i32>>,
    /// Maximum amount of time that a request should wait in the pending queue before starting a new instance to handle it.
    #[builder(into, default)]
    #[serde(rename = "maxPendingLatency")]
    pub r#max_pending_latency: Box<Option<String>>,
    /// Maximum number of instances that should be started to handle requests for this version. Default: 20
    #[builder(into, default)]
    #[serde(rename = "maxTotalInstances")]
    pub r#max_total_instances: Box<Option<i32>>,
    /// Minimum number of idle instances that should be maintained for this version. Only applicable for the default version of a service.
    #[builder(into, default)]
    #[serde(rename = "minIdleInstances")]
    pub r#min_idle_instances: Box<Option<i32>>,
    /// Minimum amount of time a request should wait in the pending queue before starting a new instance to handle it.
    #[builder(into, default)]
    #[serde(rename = "minPendingLatency")]
    pub r#min_pending_latency: Box<Option<String>>,
    /// Minimum number of running instances that should be maintained for this version. Default: 2
    #[builder(into, default)]
    #[serde(rename = "minTotalInstances")]
    pub r#min_total_instances: Box<Option<i32>>,
    /// Target scaling by network usage.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "networkUtilization")]
    pub r#network_utilization: Box<Option<super::super::types::appengine::FlexibleAppVersionAutomaticScalingNetworkUtilization>>,
    /// Target scaling by request utilization.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "requestUtilization")]
    pub r#request_utilization: Box<Option<super::super::types::appengine::FlexibleAppVersionAutomaticScalingRequestUtilization>>,
}
