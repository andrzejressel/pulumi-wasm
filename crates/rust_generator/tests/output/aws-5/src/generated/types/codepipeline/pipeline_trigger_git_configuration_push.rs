#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct PipelineTriggerGitConfigurationPush {
    /// The field that specifies to filter on branches for the push trigger configuration. A `branches` block is documented below.
    #[builder(into, default)]
    #[serde(rename = "branches")]
    pub r#branches: Box<Option<super::super::types::codepipeline::PipelineTriggerGitConfigurationPushBranches>>,
    /// The field that specifies to filter on file paths for the push trigger configuration. A `file_paths` block is documented below.
    #[builder(into, default)]
    #[serde(rename = "filePaths")]
    pub r#file_paths: Box<Option<super::super::types::codepipeline::PipelineTriggerGitConfigurationPushFilePaths>>,
    /// The field that contains the details for the Git tags trigger configuration. A `tags` block is documented below.
    #[builder(into, default)]
    #[serde(rename = "tags")]
    pub r#tags: Box<Option<super::super::types::codepipeline::PipelineTriggerGitConfigurationPushTags>>,
}
