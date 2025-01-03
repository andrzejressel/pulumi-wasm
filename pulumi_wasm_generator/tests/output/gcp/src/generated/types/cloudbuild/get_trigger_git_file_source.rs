#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetTriggerGitFileSource {
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
    /// The path of the file, with the repo root as the root of the path.
    #[builder(into)]
    #[serde(rename = "path")]
    pub r#path: Box<String>,
    /// The type of the repo, since it may not be explicit from the repo field (e.g from a URL).
    /// Values can be UNKNOWN, CLOUD_SOURCE_REPOSITORIES, GITHUB, BITBUCKET_SERVER Possible values: ["UNKNOWN", "CLOUD_SOURCE_REPOSITORIES", "GITHUB", "BITBUCKET_SERVER"]
    #[builder(into)]
    #[serde(rename = "repoType")]
    pub r#repo_type: Box<String>,
    /// The fully qualified resource name of the Repo API repository. The fully qualified resource name of the Repo API repository.
    /// If unspecified, the repo from which the trigger invocation originated is assumed to be the repo from which to read the specified path.
    #[builder(into)]
    #[serde(rename = "repository")]
    pub r#repository: Box<String>,
    /// The branch, tag, arbitrary ref, or SHA version of the repo to use when resolving the
    /// filename (optional). This field respects the same syntax/resolution as described here: https://git-scm.com/docs/gitrevisions
    /// If unspecified, the revision from which the trigger invocation originated is assumed to be the revision from which to read the specified path.
    #[builder(into)]
    #[serde(rename = "revision")]
    pub r#revision: Box<String>,
    /// The URI of the repo (optional). If unspecified, the repo from which the trigger
    /// invocation originated is assumed to be the repo from which to read the specified path.
    #[builder(into)]
    #[serde(rename = "uri")]
    pub r#uri: Box<String>,
}
