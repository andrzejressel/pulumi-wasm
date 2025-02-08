#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct TriggerSourceToBuild {
    /// The full resource name of the bitbucket server config.
    /// Format: projects/{project}/locations/{location}/bitbucketServerConfigs/{id}.
    #[builder(into, default)]
    #[serde(rename = "bitbucketServerConfig")]
    pub r#bitbucket_server_config: Box<Option<String>>,
    /// The full resource name of the github enterprise config.
    /// Format: projects/{project}/locations/{location}/githubEnterpriseConfigs/{id}. projects/{project}/githubEnterpriseConfigs/{id}.
    #[builder(into, default)]
    #[serde(rename = "githubEnterpriseConfig")]
    pub r#github_enterprise_config: Box<Option<String>>,
    /// The branch or tag to use. Must start with "refs/" (required).
    #[builder(into)]
    #[serde(rename = "ref")]
    pub r#ref_: Box<String>,
    /// The type of the repo, since it may not be explicit from the repo field (e.g from a URL).
    /// Values can be UNKNOWN, CLOUD_SOURCE_REPOSITORIES, GITHUB, BITBUCKET_SERVER
    /// Possible values are: `UNKNOWN`, `CLOUD_SOURCE_REPOSITORIES`, `GITHUB`, `BITBUCKET_SERVER`.
    #[builder(into)]
    #[serde(rename = "repoType")]
    pub r#repo_type: Box<String>,
    /// The qualified resource name of the Repo API repository.
    /// Either uri or repository can be specified and is required.
    #[builder(into, default)]
    #[serde(rename = "repository")]
    pub r#repository: Box<Option<String>>,
    /// The URI of the repo.
    #[builder(into, default)]
    #[serde(rename = "uri")]
    pub r#uri: Box<Option<String>>,
}
