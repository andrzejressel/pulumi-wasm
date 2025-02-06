#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ImagePipelineImageScanningConfigurationEcrConfiguration {
    #[builder(into, default)]
    #[serde(rename = "containerTags")]
    pub r#container_tags: Box<Option<Vec<String>>>,
    /// The name of the repository to scan
    #[builder(into, default)]
    #[serde(rename = "repositoryName")]
    pub r#repository_name: Box<Option<String>>,
}
