#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct PipelineTrigger {
    /// Provides the filter criteria and the source stage for the repository event that starts the pipeline. For more information, refer to the [AWS documentation](https://docs.aws.amazon.com/codepipeline/latest/userguide/pipelines-filter.html). A `git_configuration` block is documented below.
    #[builder(into)]
    #[serde(rename = "gitConfiguration")]
    pub r#git_configuration: Box<super::super::types::codepipeline::PipelineTriggerGitConfiguration>,
    /// The source provider for the event. Possible value is `CodeStarSourceConnection`.
    #[builder(into)]
    #[serde(rename = "providerType")]
    pub r#provider_type: Box<String>,
}
