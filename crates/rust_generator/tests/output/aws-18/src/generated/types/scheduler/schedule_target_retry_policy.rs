#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ScheduleTargetRetryPolicy {
    /// Maximum amount of time, in seconds, to continue to make retry attempts. Ranges from `60` to `86400` (default).
    #[builder(into, default)]
    #[serde(rename = "maximumEventAgeInSeconds")]
    pub r#maximum_event_age_in_seconds: Box<Option<i32>>,
    /// Maximum number of retry attempts to make before the request fails. Ranges from `0` to `185` (default).
    #[builder(into, default)]
    #[serde(rename = "maximumRetryAttempts")]
    pub r#maximum_retry_attempts: Box<Option<i32>>,
}
