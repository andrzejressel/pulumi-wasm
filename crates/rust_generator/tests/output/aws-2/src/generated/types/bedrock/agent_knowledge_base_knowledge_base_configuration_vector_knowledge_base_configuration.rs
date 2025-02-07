#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct AgentKnowledgeBaseKnowledgeBaseConfigurationVectorKnowledgeBaseConfiguration {
    /// ARN of the model used to create vector embeddings for the knowledge base.
    #[builder(into)]
    #[serde(rename = "embeddingModelArn")]
    pub r#embedding_model_arn: Box<String>,
}
