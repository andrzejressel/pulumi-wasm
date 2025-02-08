#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct GetAppServiceSourceControl {
    /// The branch of the remote repository in use.
    #[builder(into)]
    #[serde(rename = "branch")]
    pub r#branch: Box<String>,
    /// Limits to manual integration.
    #[builder(into)]
    #[serde(rename = "manualIntegration")]
    pub r#manual_integration: Box<bool>,
    /// The URL of the source code repository.
    #[builder(into)]
    #[serde(rename = "repoUrl")]
    pub r#repo_url: Box<String>,
    /// Is roll-back enabled for the repository.
    #[builder(into)]
    #[serde(rename = "rollbackEnabled")]
    pub r#rollback_enabled: Box<bool>,
    /// Uses Mercurial if `true`, otherwise uses Git.
    #[builder(into)]
    #[serde(rename = "useMercurial")]
    pub r#use_mercurial: Box<bool>,
}
