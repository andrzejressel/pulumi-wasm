#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct PipelineDefinitionPipelineObject {
    /// Configuration block for Key-value pairs that define the properties of the object. See below
    #[builder(into, default)]
    #[serde(rename = "fields")]
    pub r#fields: Box<Option<Vec<super::super::types::datapipeline::PipelineDefinitionPipelineObjectField>>>,
    /// ID of the object.
    #[builder(into)]
    #[serde(rename = "id")]
    pub r#id: Box<String>,
    /// ARN of the storage connector.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
}
