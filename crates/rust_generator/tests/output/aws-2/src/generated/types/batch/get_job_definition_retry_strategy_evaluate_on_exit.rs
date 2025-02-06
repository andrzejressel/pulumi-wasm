#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetJobDefinitionRetryStrategyEvaluateOnExit {
    /// Specifies the action to take if all of the specified conditions (onStatusReason, onReason, and onExitCode) are met. The values aren't case sensitive.
    #[builder(into)]
    #[serde(rename = "action")]
    pub r#action: Box<String>,
    /// Contains a glob pattern to match against the decimal representation of the ExitCode returned for a job.
    #[builder(into)]
    #[serde(rename = "onExitCode")]
    pub r#on_exit_code: Box<String>,
    /// Contains a glob pattern to match against the Reason returned for a job.
    #[builder(into)]
    #[serde(rename = "onReason")]
    pub r#on_reason: Box<String>,
    /// Contains a glob pattern to match against the StatusReason returned for a job.
    #[builder(into)]
    #[serde(rename = "onStatusReason")]
    pub r#on_status_reason: Box<String>,
}
