#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct PipelineParallelismConfiguration {
    /// The max number of steps that can be executed in parallel.
    #[builder(into)]
    #[serde(rename = "maxParallelExecutionSteps")]
    pub r#max_parallel_execution_steps: Box<i32>,
}
