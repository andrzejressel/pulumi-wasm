#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct AgentKnowledgeBaseKnowledgeBaseConfiguration {
    /// Type of data that the data source is converted into for the knowledge base. Valid Values: `VECTOR`.
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: Box<String>,
    /// Details about the embeddings model that'sused to convert the data source. See `vector_knowledge_base_configuration` block for details.
    #[builder(into, default)]
    #[serde(rename = "vectorKnowledgeBaseConfiguration")]
    pub r#vector_knowledge_base_configuration: Box<Option<super::super::types::bedrock::AgentKnowledgeBaseKnowledgeBaseConfigurationVectorKnowledgeBaseConfiguration>>,
}
