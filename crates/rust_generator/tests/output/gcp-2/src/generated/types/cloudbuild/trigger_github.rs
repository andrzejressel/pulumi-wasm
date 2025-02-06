#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct TriggerGithub {
    /// The resource name of the github enterprise config that should be applied to this installation.
    /// For example: "projects/{$projectId}/locations/{$locationId}/githubEnterpriseConfigs/{$configId}"
    #[builder(into, default)]
    #[serde(rename = "enterpriseConfigResourceName")]
    pub r#enterprise_config_resource_name: Box<Option<String>>,
    /// Name of the repository. For example: The name for
    /// https://github.com/googlecloudplatform/cloud-builders is "cloud-builders".
    #[builder(into, default)]
    #[serde(rename = "name")]
    pub r#name: Box<Option<String>>,
    /// Owner of the repository. For example: The owner for
    /// https://github.com/googlecloudplatform/cloud-builders is "googlecloudplatform".
    #[builder(into, default)]
    #[serde(rename = "owner")]
    pub r#owner: Box<Option<String>>,
    /// filter to match changes in pull requests. Specify only one of `pull_request` or `push`.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "pullRequest")]
    pub r#pull_request: Box<Option<super::super::types::cloudbuild::TriggerGithubPullRequest>>,
    /// filter to match changes in refs, like branches or tags. Specify only one of `pull_request` or `push`.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "push")]
    pub r#push: Box<Option<super::super::types::cloudbuild::TriggerGithubPush>>,
}
