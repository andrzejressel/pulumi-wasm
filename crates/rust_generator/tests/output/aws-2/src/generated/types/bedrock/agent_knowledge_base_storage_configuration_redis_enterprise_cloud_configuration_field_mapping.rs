#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct AgentKnowledgeBaseStorageConfigurationRedisEnterpriseCloudConfigurationFieldMapping {
    /// Name of the field in which Amazon Bedrock stores metadata about the vector store.
    #[builder(into, default)]
    #[serde(rename = "metadataField")]
    pub r#metadata_field: Box<Option<String>>,
    /// Name of the field in which Amazon Bedrock stores the raw text from your data. The text is split according to the chunking strategy you choose.
    #[builder(into, default)]
    #[serde(rename = "textField")]
    pub r#text_field: Box<Option<String>>,
    /// Name of the field in which Amazon Bedrock stores the vector embeddings for your data sources.
    #[builder(into, default)]
    #[serde(rename = "vectorField")]
    pub r#vector_field: Box<Option<String>>,
}
