#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetImagePipelineSchedule {
    /// Condition when the pipeline should trigger a new image build.
    #[builder(into)]
    #[serde(rename = "pipelineExecutionStartCondition")]
    pub r#pipeline_execution_start_condition: Box<String>,
    /// Cron expression of how often the pipeline start condition is evaluated.
    #[builder(into)]
    #[serde(rename = "scheduleExpression")]
    pub r#schedule_expression: Box<String>,
}
