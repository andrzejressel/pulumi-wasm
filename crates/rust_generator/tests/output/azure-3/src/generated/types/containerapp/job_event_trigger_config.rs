#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct JobEventTriggerConfig {
    /// Number of parallel replicas of a job that can run at a given time.
    #[builder(into, default)]
    #[serde(rename = "parallelism")]
    pub r#parallelism: Box<Option<i32>>,
    /// Minimum number of successful replica completions before overall job completion.
    #[builder(into, default)]
    #[serde(rename = "replicaCompletionCount")]
    pub r#replica_completion_count: Box<Option<i32>>,
    /// A `scale` block as defined below.
    #[builder(into, default)]
    #[serde(rename = "scales")]
    pub r#scales: Box<Option<Vec<super::super::types::containerapp::JobEventTriggerConfigScale>>>,
}
