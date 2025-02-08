#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct TaskDefinitionVolumeEfsVolumeConfiguration {
    /// Configuration block for authorization for the Amazon EFS file system. Detailed below.
    #[builder(into, default)]
    #[serde(rename = "authorizationConfig")]
    pub r#authorization_config: Box<Option<super::super::types::ecs::TaskDefinitionVolumeEfsVolumeConfigurationAuthorizationConfig>>,
    /// ID of the EFS File System.
    #[builder(into)]
    #[serde(rename = "fileSystemId")]
    pub r#file_system_id: Box<String>,
    /// Directory within the Amazon EFS file system to mount as the root directory inside the host. If this parameter is omitted, the root of the Amazon EFS volume will be used. Specifying / will have the same effect as omitting this parameter. This argument is ignored when using `authorization_config`.
    #[builder(into, default)]
    #[serde(rename = "rootDirectory")]
    pub r#root_directory: Box<Option<String>>,
    /// Whether or not to enable encryption for Amazon EFS data in transit between the Amazon ECS host and the Amazon EFS server. Transit encryption must be enabled if Amazon EFS IAM authorization is used. Valid values: `ENABLED`, `DISABLED`. If this parameter is omitted, the default value of `DISABLED` is used.
    #[builder(into, default)]
    #[serde(rename = "transitEncryption")]
    pub r#transit_encryption: Box<Option<String>>,
    /// Port to use for transit encryption. If you do not specify a transit encryption port, it will use the port selection strategy that the Amazon EFS mount helper uses.
    #[builder(into, default)]
    #[serde(rename = "transitEncryptionPort")]
    pub r#transit_encryption_port: Box<Option<i32>>,
}
