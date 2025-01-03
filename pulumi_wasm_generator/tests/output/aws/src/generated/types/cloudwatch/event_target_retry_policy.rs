#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct EventTargetRetryPolicy {
    /// The age in seconds to continue to make retry attempts.
    #[builder(into, default)]
    #[serde(rename = "maximumEventAgeInSeconds")]
    pub r#maximum_event_age_in_seconds: Box<Option<i32>>,
    /// maximum number of retry attempts to make before the request fails
    #[builder(into, default)]
    #[serde(rename = "maximumRetryAttempts")]
    pub r#maximum_retry_attempts: Box<Option<i32>>,
}
