#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct QueueRetryConfig {
    /// Number of attempts per task.
    /// Cloud Tasks will attempt the task maxAttempts times (that is, if
    /// the first attempt fails, then there will be maxAttempts - 1
    /// retries). Must be >= -1.
    /// If unspecified when the queue is created, Cloud Tasks will pick
    /// the default.
    /// -1 indicates unlimited attempts.
    #[builder(into, default)]
    #[serde(rename = "maxAttempts")]
    pub r#max_attempts: Box<Option<i32>>,
    /// A task will be scheduled for retry between minBackoff and
    /// maxBackoff duration after it fails, if the queue's RetryConfig
    /// specifies that the task should be retried.
    #[builder(into, default)]
    #[serde(rename = "maxBackoff")]
    pub r#max_backoff: Box<Option<String>>,
    /// The time between retries will double maxDoublings times.
    /// A task's retry interval starts at minBackoff, then doubles maxDoublings times,
    /// then increases linearly, and finally retries retries at intervals of maxBackoff
    /// up to maxAttempts times.
    #[builder(into, default)]
    #[serde(rename = "maxDoublings")]
    pub r#max_doublings: Box<Option<i32>>,
    /// If positive, maxRetryDuration specifies the time limit for
    /// retrying a failed task, measured from when the task was first
    /// attempted. Once maxRetryDuration time has passed and the task has
    /// been attempted maxAttempts times, no further attempts will be
    /// made and the task will be deleted.
    /// If zero, then the task age is unlimited.
    #[builder(into, default)]
    #[serde(rename = "maxRetryDuration")]
    pub r#max_retry_duration: Box<Option<String>>,
    /// A task will be scheduled for retry between minBackoff and
    /// maxBackoff duration after it fails, if the queue's RetryConfig
    /// specifies that the task should be retried.
    #[builder(into, default)]
    #[serde(rename = "minBackoff")]
    pub r#min_backoff: Box<Option<String>>,
}
