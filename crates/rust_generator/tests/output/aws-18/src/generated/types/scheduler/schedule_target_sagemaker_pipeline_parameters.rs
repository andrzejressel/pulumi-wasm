#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct ScheduleTargetSagemakerPipelineParameters {
    /// Set of up to 200 parameter names and values to use when executing the SageMaker Model Building Pipeline. Detailed below.
    #[builder(into, default)]
    #[serde(rename = "pipelineParameters")]
    pub r#pipeline_parameters: Box<Option<Vec<super::super::types::scheduler::ScheduleTargetSagemakerPipelineParametersPipelineParameter>>>,
}
