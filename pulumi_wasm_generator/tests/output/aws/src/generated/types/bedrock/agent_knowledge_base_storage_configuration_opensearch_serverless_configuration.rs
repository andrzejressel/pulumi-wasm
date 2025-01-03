#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct AgentKnowledgeBaseStorageConfigurationOpensearchServerlessConfiguration {
    /// ARN of the OpenSearch Service vector store.
    #[builder(into)]
    #[serde(rename = "collectionArn")]
    pub r#collection_arn: Box<String>,
    /// The names of the fields to which to map information about the vector store. This block supports the following arguments:
    #[builder(into, default)]
    #[serde(rename = "fieldMapping")]
    pub r#field_mapping: Box<Option<super::super::types::bedrock::AgentKnowledgeBaseStorageConfigurationOpensearchServerlessConfigurationFieldMapping>>,
    /// Name of the vector store.
    #[builder(into)]
    #[serde(rename = "vectorIndexName")]
    pub r#vector_index_name: Box<String>,
}
