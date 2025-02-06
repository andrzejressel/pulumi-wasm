#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetJobDefinitionNodePropertyNodeRangePropertyContainerVolume {
    /// This parameter is specified when you're using an Amazon Elastic File System file system for job storage.
    #[builder(into)]
    #[serde(rename = "efsVolumeConfigurations")]
    pub r#efs_volume_configurations: Box<Vec<super::super::types::batch::GetJobDefinitionNodePropertyNodeRangePropertyContainerVolumeEfsVolumeConfiguration>>,
    /// The contents of the host parameter determine whether your data volume persists on the host container instance and where it's stored.
    #[builder(into)]
    #[serde(rename = "hosts")]
    pub r#hosts: Box<Vec<super::super::types::batch::GetJobDefinitionNodePropertyNodeRangePropertyContainerVolumeHost>>,
    /// The name of the job definition to register. It can be up to 128 letters long. It can contain uppercase and lowercase letters, numbers, hyphens (-), and underscores (_).
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
}
