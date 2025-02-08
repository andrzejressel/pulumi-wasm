#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct QueueRateLimits {
    /// (Output)
    /// The max burst size.
    /// Max burst size limits how fast tasks in queue are processed when many tasks are
    /// in the queue and the rate is high. This field allows the queue to have a high
    /// rate so processing starts shortly after a task is enqueued, but still limits
    /// resource usage when many tasks are enqueued in a short period of time.
    #[builder(into, default)]
    #[serde(rename = "maxBurstSize")]
    pub r#max_burst_size: Box<Option<i32>>,
    /// The maximum number of concurrent tasks that Cloud Tasks allows to
    /// be dispatched for this queue. After this threshold has been
    /// reached, Cloud Tasks stops dispatching tasks until the number of
    /// concurrent requests decreases.
    #[builder(into, default)]
    #[serde(rename = "maxConcurrentDispatches")]
    pub r#max_concurrent_dispatches: Box<Option<i32>>,
    /// The maximum rate at which tasks are dispatched from this queue.
    /// If unspecified when the queue is created, Cloud Tasks will pick the default.
    #[builder(into, default)]
    #[serde(rename = "maxDispatchesPerSecond")]
    pub r#max_dispatches_per_second: Box<Option<f64>>,
}
