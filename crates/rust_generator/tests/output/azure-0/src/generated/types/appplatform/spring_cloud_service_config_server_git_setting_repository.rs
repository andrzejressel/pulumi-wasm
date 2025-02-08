#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct SpringCloudServiceConfigServerGitSettingRepository {
    /// A `http_basic_auth` block as defined below.
    #[builder(into, default)]
    #[serde(rename = "httpBasicAuth")]
    pub r#http_basic_auth: Box<Option<super::super::types::appplatform::SpringCloudServiceConfigServerGitSettingRepositoryHttpBasicAuth>>,
    /// The default label of the Git repository, should be the branch name, tag name, or commit-id of the repository.
    #[builder(into, default)]
    #[serde(rename = "label")]
    pub r#label: Box<Option<String>>,
    /// A name to identify on the Git repository, required only if repos exists.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// An array of strings used to match an application name. For each pattern, use the `{application}/{profile}` format with wildcards.
    #[builder(into, default)]
    #[serde(rename = "patterns")]
    pub r#patterns: Box<Option<Vec<String>>>,
    /// An array of strings used to search subdirectories of the Git repository.
    #[builder(into, default)]
    #[serde(rename = "searchPaths")]
    pub r#search_paths: Box<Option<Vec<String>>>,
    /// A `ssh_auth` block as defined below.
    #[builder(into, default)]
    #[serde(rename = "sshAuth")]
    pub r#ssh_auth: Box<Option<super::super::types::appplatform::SpringCloudServiceConfigServerGitSettingRepositorySshAuth>>,
    /// The URI of the Git repository that's used as the Config Server back end should be started with `http://`, `https://`, `git@`, or `ssh://`.
    #[builder(into)]
    #[serde(rename = "uri")]
    pub r#uri: Box<String>,
}
