#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetComputeEnvironmentUpdatePolicy {
    #[builder(into)]
    #[serde(rename = "jobExecutionTimeoutMinutes")]
    pub r#job_execution_timeout_minutes: Box<i32>,
    #[builder(into)]
    #[serde(rename = "terminateJobsOnUpdate")]
    pub r#terminate_jobs_on_update: Box<bool>,
}