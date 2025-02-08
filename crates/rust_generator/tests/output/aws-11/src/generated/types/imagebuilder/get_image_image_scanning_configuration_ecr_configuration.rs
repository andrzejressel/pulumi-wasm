#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct GetImageImageScanningConfigurationEcrConfiguration {
    /// Set of tags for Image Builder to apply to the output container image that that Amazon Inspector scans.
    #[builder(into)]
    #[serde(rename = "containerTags")]
    pub r#container_tags: Box<Vec<String>>,
    /// The name of the container repository that Amazon Inspector scans to identify findings for your container images.
    #[builder(into)]
    #[serde(rename = "repositoryName")]
    pub r#repository_name: Box<String>,
}
