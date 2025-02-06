#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct InstanceGceSetupContainerImage {
    /// The path to the container image repository.
    /// For example: gcr.io/{project_id}/{imageName}
    #[builder(into)]
    #[serde(rename = "repository")]
    pub r#repository: Box<String>,
    /// The tag of the container image. If not specified, this defaults to the latest tag.
    #[builder(into, default)]
    #[serde(rename = "tag")]
    pub r#tag: Box<Option<String>>,
}
