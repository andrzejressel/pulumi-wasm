#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct GetPipelineDefinitionParameterObject {
    #[builder(into)]
    #[serde(rename = "attributes")]
    pub r#attributes: Box<Vec<super::super::types::datapipeline::GetPipelineDefinitionParameterObjectAttribute>>,
    /// ID of the object.
    #[builder(into)]
    #[serde(rename = "id")]
    pub r#id: Box<String>,
}
