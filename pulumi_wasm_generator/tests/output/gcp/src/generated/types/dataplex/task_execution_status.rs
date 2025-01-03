#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct TaskExecutionStatus {
    /// (Output)
    /// latest job execution.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "latestJobs")]
    pub r#latest_jobs: Box<Option<Vec<super::super::types::dataplex::TaskExecutionStatusLatestJob>>>,
    /// (Output)
    /// Last update time of the status.
    #[builder(into, default)]
    #[serde(rename = "updateTime")]
    pub r#update_time: Box<Option<String>>,
}
