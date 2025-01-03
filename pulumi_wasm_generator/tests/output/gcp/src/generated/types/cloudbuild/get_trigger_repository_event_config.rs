#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetTriggerRepositoryEventConfig {
    /// Contains filter properties for matching Pull Requests.
    #[builder(into)]
    #[serde(rename = "pullRequests")]
    pub r#pull_requests: Box<Vec<super::super::types::cloudbuild::GetTriggerRepositoryEventConfigPullRequest>>,
    /// Contains filter properties for matching git pushes.
    #[builder(into)]
    #[serde(rename = "pushes")]
    pub r#pushes: Box<Vec<super::super::types::cloudbuild::GetTriggerRepositoryEventConfigPush>>,
    /// The resource name of the Repo API resource.
    #[builder(into)]
    #[serde(rename = "repository")]
    pub r#repository: Box<String>,
}
