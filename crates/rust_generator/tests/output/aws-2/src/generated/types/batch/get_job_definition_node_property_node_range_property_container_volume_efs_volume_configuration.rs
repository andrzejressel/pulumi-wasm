#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct GetJobDefinitionNodePropertyNodeRangePropertyContainerVolumeEfsVolumeConfiguration {
    /// The authorization configuration details for the Amazon EFS file system.
    #[builder(into)]
    #[serde(rename = "authorizationConfigs")]
    pub r#authorization_configs: Box<Vec<super::super::types::batch::GetJobDefinitionNodePropertyNodeRangePropertyContainerVolumeEfsVolumeConfigurationAuthorizationConfig>>,
    /// The Amazon EFS file system ID to use.
    #[builder(into)]
    #[serde(rename = "fileSystemId")]
    pub r#file_system_id: Box<String>,
    /// The directory within the Amazon EFS file system to mount as the root directory inside the host.
    #[builder(into)]
    #[serde(rename = "rootDirectory")]
    pub r#root_directory: Box<String>,
    /// Determines whether to enable encryption for Amazon EFS data in transit between the Amazon ECS host and the Amazon EFS server
    #[builder(into)]
    #[serde(rename = "transitEncryption")]
    pub r#transit_encryption: Box<String>,
    /// The port to use when sending encrypted data between the Amazon ECS host and the Amazon EFS server.
    #[builder(into)]
    #[serde(rename = "transitEncryptionPort")]
    pub r#transit_encryption_port: Box<i32>,
}
