#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct PipeTargetParametersSagemakerPipelineParameters {
    /// List of Parameter names and values for SageMaker Model Building Pipeline execution. Detailed below.
    #[builder(into, default)]
    #[serde(rename = "pipelineParameters")]
    pub r#pipeline_parameters: Box<Option<Vec<super::super::types::pipes::PipeTargetParametersSagemakerPipelineParametersPipelineParameter>>>,
}
