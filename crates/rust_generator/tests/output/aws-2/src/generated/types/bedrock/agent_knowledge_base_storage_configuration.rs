#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct AgentKnowledgeBaseStorageConfiguration {
    /// The storage configuration of the knowledge base in Amazon OpenSearch Service. See `opensearch_serverless_configuration` block for details.
    #[builder(into, default)]
    #[serde(rename = "opensearchServerlessConfiguration")]
    pub r#opensearch_serverless_configuration: Box<Option<super::super::types::bedrock::AgentKnowledgeBaseStorageConfigurationOpensearchServerlessConfiguration>>,
    /// The storage configuration of the knowledge base in Pinecone. See `pinecone_configuration` block for details.
    #[builder(into, default)]
    #[serde(rename = "pineconeConfiguration")]
    pub r#pinecone_configuration: Box<Option<super::super::types::bedrock::AgentKnowledgeBaseStorageConfigurationPineconeConfiguration>>,
    /// Details about the storage configuration of the knowledge base in Amazon RDS. For more information, see [Create a vector index in Amazon RDS](https://docs.aws.amazon.com/bedrock/latest/userguide/knowledge-base-setup.html). See `rds_configuration` block for details.
    #[builder(into, default)]
    #[serde(rename = "rdsConfiguration")]
    pub r#rds_configuration: Box<Option<super::super::types::bedrock::AgentKnowledgeBaseStorageConfigurationRdsConfiguration>>,
    /// The storage configuration of the knowledge base in Redis Enterprise Cloud. See `redis_enterprise_cloud_configuration` block for details.
    #[builder(into, default)]
    #[serde(rename = "redisEnterpriseCloudConfiguration")]
    pub r#redis_enterprise_cloud_configuration: Box<Option<super::super::types::bedrock::AgentKnowledgeBaseStorageConfigurationRedisEnterpriseCloudConfiguration>>,
    /// Vector store service in which the knowledge base is stored. Valid Values: `OPENSEARCH_SERVERLESS`, `PINECONE`, `REDIS_ENTERPRISE_CLOUD`, `RDS`.
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: Box<String>,
}
