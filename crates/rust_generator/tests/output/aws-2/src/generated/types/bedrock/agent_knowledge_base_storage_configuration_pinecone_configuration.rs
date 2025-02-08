#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct AgentKnowledgeBaseStorageConfigurationPineconeConfiguration {
    /// Endpoint URL for your index management page.
    #[builder(into)]
    #[serde(rename = "connectionString")]
    pub r#connection_string: Box<String>,
    /// ARN of the secret that you created in AWS Secrets Manager that is linked to your Pinecone API key.
    #[builder(into)]
    #[serde(rename = "credentialsSecretArn")]
    pub r#credentials_secret_arn: Box<String>,
    /// The names of the fields to which to map information about the vector store. This block supports the following arguments:
    #[builder(into, default)]
    #[serde(rename = "fieldMapping")]
    pub r#field_mapping: Box<Option<super::super::types::bedrock::AgentKnowledgeBaseStorageConfigurationPineconeConfigurationFieldMapping>>,
    /// Namespace to be used to write new data to your database.
    #[builder(into, default)]
    #[serde(rename = "namespace")]
    pub r#namespace: Box<Option<String>>,
}
