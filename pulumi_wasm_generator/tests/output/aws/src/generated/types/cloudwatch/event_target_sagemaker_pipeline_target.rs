#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct EventTargetSagemakerPipelineTarget {
    /// List of Parameter names and values for SageMaker Model Building Pipeline execution.
    #[builder(into, default)]
    #[serde(rename = "pipelineParameterLists")]
    pub r#pipeline_parameter_lists: Box<Option<Vec<super::super::types::cloudwatch::EventTargetSagemakerPipelineTargetPipelineParameterList>>>,
}