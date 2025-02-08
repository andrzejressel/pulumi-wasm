#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct GetJobDefinitionEksPropertyPodPropertyVolume {
    /// Specifies the configuration of a Kubernetes emptyDir volume.
    #[builder(into)]
    #[serde(rename = "emptyDirs")]
    pub r#empty_dirs: Box<Vec<super::super::types::batch::GetJobDefinitionEksPropertyPodPropertyVolumeEmptyDir>>,
    /// The path for the device on the host container instance.
    #[builder(into)]
    #[serde(rename = "hostPaths")]
    pub r#host_paths: Box<Vec<super::super::types::batch::GetJobDefinitionEksPropertyPodPropertyVolumeHostPath>>,
    /// The name of the job definition to register. It can be up to 128 letters long. It can contain uppercase and lowercase letters, numbers, hyphens (-), and underscores (_).
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// Specifies the configuration of a Kubernetes secret volume.
    #[builder(into)]
    #[serde(rename = "secrets")]
    pub r#secrets: Box<Vec<super::super::types::batch::GetJobDefinitionEksPropertyPodPropertyVolumeSecret>>,
}
