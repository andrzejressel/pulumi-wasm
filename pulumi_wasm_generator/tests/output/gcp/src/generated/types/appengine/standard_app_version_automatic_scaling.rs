#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct StandardAppVersionAutomaticScaling {
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
    /// A duration in seconds with up to nine fractional digits, terminated by 's'. Example: "3.5s".
    #[builder(into, default)]
    #[serde(rename = "maxPendingLatency")]
    pub r#max_pending_latency: Box<Option<String>>,
    /// Minimum number of idle instances that should be maintained for this version. Only applicable for the default version of a service.
    #[builder(into, default)]
    #[serde(rename = "minIdleInstances")]
    pub r#min_idle_instances: Box<Option<i32>>,
    /// Minimum amount of time a request should wait in the pending queue before starting a new instance to handle it.
    /// A duration in seconds with up to nine fractional digits, terminated by 's'. Example: "3.5s".
    #[builder(into, default)]
    #[serde(rename = "minPendingLatency")]
    pub r#min_pending_latency: Box<Option<String>>,
    /// Scheduler settings for standard environment.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "standardSchedulerSettings")]
    pub r#standard_scheduler_settings: Box<Option<super::super::types::appengine::StandardAppVersionAutomaticScalingStandardSchedulerSettings>>,
}
