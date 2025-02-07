#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct PipelineDefinitionParameterObject {
    /// Configuration block for attributes of the parameter object. See below
    #[builder(into, default)]
    #[serde(rename = "attributes")]
    pub r#attributes: Box<Option<Vec<super::super::types::datapipeline::PipelineDefinitionParameterObjectAttribute>>>,
    /// ID of the parameter object.
    #[builder(into)]
    #[serde(rename = "id")]
    pub r#id: Box<String>,
}
