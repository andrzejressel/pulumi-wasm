#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct JobDefinitionRetryStrategyEvaluateOnExit {
    /// Action to take if all of the specified conditions are met. The values are not case sensitive. Valid values: `retry`, `exit`.
    #[builder(into)]
    #[serde(rename = "action")]
    pub r#action: Box<String>,
    /// Glob pattern to match against the decimal representation of the exit code returned for a job.
    #[builder(into, default)]
    #[serde(rename = "onExitCode")]
    pub r#on_exit_code: Box<Option<String>>,
    /// Glob pattern to match against the reason returned for a job.
    #[builder(into, default)]
    #[serde(rename = "onReason")]
    pub r#on_reason: Box<Option<String>>,
    /// Glob pattern to match against the status reason returned for a job.
    #[builder(into, default)]
    #[serde(rename = "onStatusReason")]
    pub r#on_status_reason: Box<Option<String>>,
}
