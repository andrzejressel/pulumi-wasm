#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetRepositoryDockerConfig {
    /// The repository which enabled this flag prevents all tags from being modified, moved or deleted. This does not prevent tags from being created.
    #[builder(into)]
    #[serde(rename = "immutableTags")]
    pub r#immutable_tags: Box<bool>,
}
