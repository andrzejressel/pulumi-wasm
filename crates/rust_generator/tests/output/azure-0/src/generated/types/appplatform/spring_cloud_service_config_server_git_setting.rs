#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct SpringCloudServiceConfigServerGitSetting {
    /// A `http_basic_auth` block as defined below.
    #[builder(into, default)]
    #[serde(rename = "httpBasicAuth")]
    pub r#http_basic_auth: Box<Option<super::super::types::appplatform::SpringCloudServiceConfigServerGitSettingHttpBasicAuth>>,
    /// The default label of the Git repository, should be the branch name, tag name, or commit-id of the repository.
    #[builder(into, default)]
    #[serde(rename = "label")]
    pub r#label: Box<Option<String>>,
    /// One or more `repository` blocks as defined below.
    #[builder(into, default)]
    #[serde(rename = "repositories")]
    pub r#repositories: Box<Option<Vec<super::super::types::appplatform::SpringCloudServiceConfigServerGitSettingRepository>>>,
    /// An array of strings used to search subdirectories of the Git repository.
    #[builder(into, default)]
    #[serde(rename = "searchPaths")]
    pub r#search_paths: Box<Option<Vec<String>>>,
    /// A `ssh_auth` block as defined below.
    #[builder(into, default)]
    #[serde(rename = "sshAuth")]
    pub r#ssh_auth: Box<Option<super::super::types::appplatform::SpringCloudServiceConfigServerGitSettingSshAuth>>,
    /// The URI of the default Git repository used as the Config Server back end, should be started with `http://`, `https://`, `git@`, or `ssh://`.
    #[builder(into)]
    #[serde(rename = "uri")]
    pub r#uri: Box<String>,
}
