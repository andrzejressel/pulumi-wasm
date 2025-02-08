#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ServiceSourceConfigurationCodeRepositorySourceCodeVersion {
    /// Type of version identifier. For a git-based repository, branches represent versions. Valid values: `BRANCH`.
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: Box<String>,
    /// Source code version. For a git-based repository, a branch name maps to a specific version. App Runner uses the most recent commit to the branch.
    #[builder(into)]
    #[serde(rename = "value")]
    pub r#value: Box<String>,
}
