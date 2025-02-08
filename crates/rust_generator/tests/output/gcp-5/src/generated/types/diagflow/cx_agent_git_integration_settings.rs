#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct CxAgentGitIntegrationSettings {
    /// Settings of integration with GitHub.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "githubSettings")]
    pub r#github_settings: Box<Option<super::super::types::diagflow::CxAgentGitIntegrationSettingsGithubSettings>>,
}
