#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetFactoryGithubConfiguration {
    /// The VSTS account name.
    #[builder(into)]
    #[serde(rename = "accountName")]
    pub r#account_name: Box<String>,
    /// The branch of the repository to get code from.
    #[builder(into)]
    #[serde(rename = "branchName")]
    pub r#branch_name: Box<String>,
    /// The GitHub repository url.
    #[builder(into)]
    #[serde(rename = "gitUrl")]
    pub r#git_url: Box<String>,
    /// The name of the git repository.
    #[builder(into)]
    #[serde(rename = "repositoryName")]
    pub r#repository_name: Box<String>,
    /// The root folder within the repository.
    #[builder(into)]
    #[serde(rename = "rootFolder")]
    pub r#root_folder: Box<String>,
}
