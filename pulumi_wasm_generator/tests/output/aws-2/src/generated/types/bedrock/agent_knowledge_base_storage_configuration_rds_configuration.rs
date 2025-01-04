#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct AgentKnowledgeBaseStorageConfigurationRdsConfiguration {
    /// ARN of the secret that you created in AWS Secrets Manager that is linked to your Amazon RDS database.
    #[builder(into)]
    #[serde(rename = "credentialsSecretArn")]
    pub r#credentials_secret_arn: Box<String>,
    /// Name of your Amazon RDS database.
    #[builder(into)]
    #[serde(rename = "databaseName")]
    pub r#database_name: Box<String>,
    /// Names of the fields to which to map information about the vector store. This block supports the following arguments:
    #[builder(into, default)]
    #[serde(rename = "fieldMapping")]
    pub r#field_mapping: Box<Option<super::super::types::bedrock::AgentKnowledgeBaseStorageConfigurationRdsConfigurationFieldMapping>>,
    /// ARN of the vector store.
    #[builder(into)]
    #[serde(rename = "resourceArn")]
    pub r#resource_arn: Box<String>,
    /// Name of the table in the database.
    #[builder(into)]
    #[serde(rename = "tableName")]
    pub r#table_name: Box<String>,
}
