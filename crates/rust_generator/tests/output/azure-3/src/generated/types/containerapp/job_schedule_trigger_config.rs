#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct JobScheduleTriggerConfig {
    /// Cron formatted repeating schedule of a Cron Job.
    #[builder(into)]
    #[serde(rename = "cronExpression")]
    pub r#cron_expression: Box<String>,
    /// Number of parallel replicas of a job that can run at a given time.
    #[builder(into, default)]
    #[serde(rename = "parallelism")]
    pub r#parallelism: Box<Option<i32>>,
    /// Minimum number of successful replica completions before overall job completion.
    #[builder(into, default)]
    #[serde(rename = "replicaCompletionCount")]
    pub r#replica_completion_count: Box<Option<i32>>,
}
