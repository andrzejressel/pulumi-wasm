#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetSpringCloudServiceConfigServerGitSetting {
    /// A `http_basic_auth` block as defined below.
    #[builder(into)]
    #[serde(rename = "httpBasicAuths")]
    pub r#http_basic_auths: Box<Vec<super::super::types::appplatform::GetSpringCloudServiceConfigServerGitSettingHttpBasicAuth>>,
    /// The default label of the Git repository, which is a branch name, tag name, or commit-id of the repository
    #[builder(into)]
    #[serde(rename = "label")]
    pub r#label: Box<String>,
    /// One or more `repository` blocks as defined below.
    #[builder(into)]
    #[serde(rename = "repositories")]
    pub r#repositories: Box<Vec<super::super::types::appplatform::GetSpringCloudServiceConfigServerGitSettingRepository>>,
    /// An array of strings used to search subdirectories of the Git repository.
    #[builder(into)]
    #[serde(rename = "searchPaths")]
    pub r#search_paths: Box<Vec<String>>,
    /// A `ssh_auth` block as defined below.
    #[builder(into)]
    #[serde(rename = "sshAuths")]
    pub r#ssh_auths: Box<Vec<super::super::types::appplatform::GetSpringCloudServiceConfigServerGitSettingSshAuth>>,
    /// The URI of the Git repository
    #[builder(into)]
    #[serde(rename = "uri")]
    pub r#uri: Box<String>,
}
