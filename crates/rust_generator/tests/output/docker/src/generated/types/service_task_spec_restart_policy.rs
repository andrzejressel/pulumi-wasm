#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ServiceTaskSpecRestartPolicy {
    /// Condition for restart
    #[builder(into, default)]
    #[serde(rename = "condition")]
    pub r#condition: Box<Option<String>>,
    /// Delay between restart attempts (ms|s|m|h)
    #[builder(into, default)]
    #[serde(rename = "delay")]
    pub r#delay: Box<Option<String>>,
    /// Maximum attempts to restart a given container before giving up (default value is `0`, which is ignored)
    #[builder(into, default)]
    #[serde(rename = "maxAttempts")]
    pub r#max_attempts: Box<Option<i32>>,
    /// The time window used to evaluate the restart policy (default value is `0`, which is unbounded) (ms|s|m|h)
    #[builder(into, default)]
    #[serde(rename = "window")]
    pub r#window: Box<Option<String>>,
}
