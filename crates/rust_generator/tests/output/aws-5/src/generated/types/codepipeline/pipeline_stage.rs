#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct PipelineStage {
    /// The action(s) to include in the stage. Defined as an `action` block below
    #[builder(into)]
    #[serde(rename = "actions")]
    pub r#actions: Box<Vec<super::super::types::codepipeline::PipelineStageAction>>,
    /// The name of the stage.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
}
