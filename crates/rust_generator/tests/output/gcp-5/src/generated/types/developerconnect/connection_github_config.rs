#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ConnectionGithubConfig {
    /// Optional. GitHub App installation id.
    #[builder(into, default)]
    #[serde(rename = "appInstallationId")]
    pub r#app_installation_id: Box<Option<String>>,
    /// Represents an OAuth token of the account that authorized the Connection,and
    /// associated metadata.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "authorizerCredential")]
    pub r#authorizer_credential: Box<Option<super::super::types::developerconnect::ConnectionGithubConfigAuthorizerCredential>>,
    /// Required. Immutable. The GitHub Application that was installed to
    /// the GitHub user or organization.
    /// Possible values:
    /// GIT_HUB_APP_UNSPECIFIED
    /// DEVELOPER_CONNECT
    /// FIREBASE"
    #[builder(into)]
    #[serde(rename = "githubApp")]
    pub r#github_app: Box<String>,
    /// (Output)
    /// Output only. The URI to navigate to in order to manage the installation
    /// associated with this GitHubConfig.
    #[builder(into, default)]
    #[serde(rename = "installationUri")]
    pub r#installation_uri: Box<Option<String>>,
}
