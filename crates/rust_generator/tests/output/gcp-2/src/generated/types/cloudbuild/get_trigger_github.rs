#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetTriggerGithub {
    /// The resource name of the github enterprise config that should be applied to this installation.
    /// For example: "projects/{$projectId}/locations/{$locationId}/githubEnterpriseConfigs/{$configId}"
    #[builder(into)]
    #[serde(rename = "enterpriseConfigResourceName")]
    pub r#enterprise_config_resource_name: Box<String>,
    /// Name of the repository. For example: The name for
    /// https://github.com/googlecloudplatform/cloud-builders is "cloud-builders".
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// Owner of the repository. For example: The owner for
    /// https://github.com/googlecloudplatform/cloud-builders is "googlecloudplatform".
    #[builder(into)]
    #[serde(rename = "owner")]
    pub r#owner: Box<String>,
    /// filter to match changes in pull requests. Specify only one of 'pull_request' or 'push'.
    #[builder(into)]
    #[serde(rename = "pullRequests")]
    pub r#pull_requests: Box<Vec<super::super::types::cloudbuild::GetTriggerGithubPullRequest>>,
    /// filter to match changes in refs, like branches or tags. Specify only one of 'pull_request' or 'push'.
    #[builder(into)]
    #[serde(rename = "pushes")]
    pub r#pushes: Box<Vec<super::super::types::cloudbuild::GetTriggerGithubPush>>,
}
