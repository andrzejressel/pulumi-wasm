#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetTriggerSourceToBuild {
    /// The full resource name of the bitbucket server config.
    /// Format: projects/{project}/locations/{location}/bitbucketServerConfigs/{id}.
    #[builder(into)]
    #[serde(rename = "bitbucketServerConfig")]
    pub r#bitbucket_server_config: Box<String>,
    /// The full resource name of the github enterprise config.
    /// Format: projects/{project}/locations/{location}/githubEnterpriseConfigs/{id}. projects/{project}/githubEnterpriseConfigs/{id}.
    #[builder(into)]
    #[serde(rename = "githubEnterpriseConfig")]
    pub r#github_enterprise_config: Box<String>,
    /// The branch or tag to use. Must start with "refs/" (required).
    #[builder(into)]
    #[serde(rename = "ref")]
    pub r#ref_: Box<String>,
    /// The type of the repo, since it may not be explicit from the repo field (e.g from a URL).
    /// Values can be UNKNOWN, CLOUD_SOURCE_REPOSITORIES, GITHUB, BITBUCKET_SERVER Possible values: ["UNKNOWN", "CLOUD_SOURCE_REPOSITORIES", "GITHUB", "BITBUCKET_SERVER"]
    #[builder(into)]
    #[serde(rename = "repoType")]
    pub r#repo_type: Box<String>,
    /// The qualified resource name of the Repo API repository.
    /// Either uri or repository can be specified and is required.
    #[builder(into)]
    #[serde(rename = "repository")]
    pub r#repository: Box<String>,
    /// The URI of the repo.
    #[builder(into)]
    #[serde(rename = "uri")]
    pub r#uri: Box<String>,
}
