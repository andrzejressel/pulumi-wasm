#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct WorkspaceGithubRepo {
    /// Specifies the GitHub account name.
    #[builder(into)]
    #[serde(rename = "accountName")]
    pub r#account_name: Box<String>,
    /// Specifies the collaboration branch of the repository to get code from.
    #[builder(into)]
    #[serde(rename = "branchName")]
    pub r#branch_name: Box<String>,
    /// Specifies the GitHub Enterprise host name. For example: <https://github.mydomain.com>.
    /// 
    /// > **Note:** You must log in to the Synapse UI to complete the authentication to the GitHub repository.
    #[builder(into, default)]
    #[serde(rename = "gitUrl")]
    pub r#git_url: Box<Option<String>>,
    /// The last commit ID.
    #[builder(into, default)]
    #[serde(rename = "lastCommitId")]
    pub r#last_commit_id: Box<Option<String>>,
    /// Specifies the name of the git repository.
    #[builder(into)]
    #[serde(rename = "repositoryName")]
    pub r#repository_name: Box<String>,
    /// Specifies the root folder within the repository. Set to `/` for the top level.
    #[builder(into)]
    #[serde(rename = "rootFolder")]
    pub r#root_folder: Box<String>,
}
