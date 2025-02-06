#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct RepositoryGitRemoteSettings {
    /// The name of the Secret Manager secret version to use as an authentication token for Git operations. This secret is for assigning with HTTPS only(for SSH use `ssh_authentication_config`). Must be in the format projects/*/secrets/*/versions/*.
    #[builder(into, default)]
    #[serde(rename = "authenticationTokenSecretVersion")]
    pub r#authentication_token_secret_version: Box<Option<String>>,
    /// The Git remote's default branch name.
    #[builder(into)]
    #[serde(rename = "defaultBranch")]
    pub r#default_branch: Box<String>,
    /// Authentication fields for remote uris using SSH protocol.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "sshAuthenticationConfig")]
    pub r#ssh_authentication_config: Box<Option<super::super::types::dataform::RepositoryGitRemoteSettingsSshAuthenticationConfig>>,
    /// (Output)
    /// Indicates the status of the Git access token. https://cloud.google.com/dataform/reference/rest/v1beta1/projects.locations.repositories#TokenStatus
    #[builder(into, default)]
    #[serde(rename = "tokenStatus")]
    pub r#token_status: Box<Option<String>>,
    /// The Git remote's URL.
    #[builder(into)]
    #[serde(rename = "url")]
    pub r#url: Box<String>,
}
