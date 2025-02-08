#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetContainerRecipeTargetRepository {
    /// Name of the container repository where the output container image is stored. The name is prefixed by the repository location.
    #[builder(into)]
    #[serde(rename = "repositoryName")]
    pub r#repository_name: Box<String>,
    /// Service in which this image is registered.
    #[builder(into)]
    #[serde(rename = "service")]
    pub r#service: Box<String>,
}
