#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct PipelineTriggerGitConfiguration {
    /// The field where the repository event that will start the pipeline is specified as pull requests. A `pull_request` block is documented below.
    #[builder(into, default)]
    #[serde(rename = "pullRequests")]
    pub r#pull_requests: Box<Option<Vec<super::super::types::codepipeline::PipelineTriggerGitConfigurationPullRequest>>>,
    /// The field where the repository event that will start the pipeline, such as pushing Git tags, is specified with details. A `push` block is documented below.
    #[builder(into, default)]
    #[serde(rename = "pushes")]
    pub r#pushes: Box<Option<Vec<super::super::types::codepipeline::PipelineTriggerGitConfigurationPush>>>,
    /// The name of the pipeline source action where the trigger configuration, such as Git tags, is specified. The trigger configuration will start the pipeline upon the specified change only.
    #[builder(into)]
    #[serde(rename = "sourceActionName")]
    pub r#source_action_name: Box<String>,
}
