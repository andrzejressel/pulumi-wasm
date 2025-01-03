#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetPipelineDefinitionPipelineObject {
    /// Key-value pairs that define the properties of the object. See below
    #[builder(into, default)]
    #[serde(rename = "fields")]
    pub r#fields: Box<Option<Vec<super::super::types::datapipeline::GetPipelineDefinitionPipelineObjectField>>>,
    /// ID of the object.
    #[builder(into)]
    #[serde(rename = "id")]
    pub r#id: Box<String>,
    /// ARN of the storage connector.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
}
