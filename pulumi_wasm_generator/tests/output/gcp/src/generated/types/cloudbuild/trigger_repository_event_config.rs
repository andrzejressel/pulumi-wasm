#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct TriggerRepositoryEventConfig {
    /// Contains filter properties for matching Pull Requests.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "pullRequest")]
    pub r#pull_request: Box<Option<super::super::types::cloudbuild::TriggerRepositoryEventConfigPullRequest>>,
    /// Contains filter properties for matching git pushes.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "push")]
    pub r#push: Box<Option<super::super::types::cloudbuild::TriggerRepositoryEventConfigPush>>,
    /// The resource name of the Repo API resource.
    #[builder(into, default)]
    #[serde(rename = "repository")]
    pub r#repository: Box<Option<String>>,
}
