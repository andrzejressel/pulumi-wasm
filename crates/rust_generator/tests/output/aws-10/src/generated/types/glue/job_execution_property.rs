#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct JobExecutionProperty {
    /// The maximum number of concurrent runs allowed for a job. The default is 1.
    #[builder(into, default)]
    #[serde(rename = "maxConcurrentRuns")]
    pub r#max_concurrent_runs: Box<Option<i32>>,
}
