#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct TaskSparkInfrastructureSpecBatch {
    /// Total number of job executors. Executor Count should be between 2 and 100. [Default=2]
    #[builder(into, default)]
    #[serde(rename = "executorsCount")]
    pub r#executors_count: Box<Option<i32>>,
    /// Max configurable executors. If maxExecutorsCount > executorsCount, then auto-scaling is enabled. Max Executor Count should be between 2 and 1000. [Default=1000]
    #[builder(into, default)]
    #[serde(rename = "maxExecutorsCount")]
    pub r#max_executors_count: Box<Option<i32>>,
}
