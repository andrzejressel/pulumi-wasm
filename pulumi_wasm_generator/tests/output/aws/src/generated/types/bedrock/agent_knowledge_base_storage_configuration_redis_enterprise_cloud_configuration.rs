#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct AgentKnowledgeBaseStorageConfigurationRedisEnterpriseCloudConfiguration {
    /// ARN of the secret that you created in AWS Secrets Manager that is linked to your Redis Enterprise Cloud database.
    #[builder(into)]
    #[serde(rename = "credentialsSecretArn")]
    pub r#credentials_secret_arn: Box<String>,
    /// Endpoint URL of the Redis Enterprise Cloud database.
    #[builder(into)]
    #[serde(rename = "endpoint")]
    pub r#endpoint: Box<String>,
    /// The names of the fields to which to map information about the vector store. This block supports the following arguments:
    #[builder(into, default)]
    #[serde(rename = "fieldMapping")]
    pub r#field_mapping: Box<Option<super::super::types::bedrock::AgentKnowledgeBaseStorageConfigurationRedisEnterpriseCloudConfigurationFieldMapping>>,
    /// Name of the vector index.
    #[builder(into)]
    #[serde(rename = "vectorIndexName")]
    pub r#vector_index_name: Box<String>,
}
