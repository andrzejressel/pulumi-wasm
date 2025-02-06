#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct JobQueueJobStateTimeLimitAction {
    /// The action to take when a job is at the head of the job queue in the specified state for the specified period of time. Valid values include `"CANCEL"`
    /// * `job_state_time_limit_action.#.max_time_seconds` - The approximate amount of time, in seconds, that must pass with the job in the specified state before the action is taken. Valid values include integers between `600` & `86400`
    #[builder(into)]
    #[serde(rename = "action")]
    pub r#action: Box<String>,
    #[builder(into)]
    #[serde(rename = "maxTimeSeconds")]
    pub r#max_time_seconds: Box<i32>,
    /// The reason to log for the action being taken.
    #[builder(into)]
    #[serde(rename = "reason")]
    pub r#reason: Box<String>,
    /// The state of the job needed to trigger the action. Valid values include `"RUNNABLE"`.
    #[builder(into)]
    #[serde(rename = "state")]
    pub r#state: Box<String>,
}
