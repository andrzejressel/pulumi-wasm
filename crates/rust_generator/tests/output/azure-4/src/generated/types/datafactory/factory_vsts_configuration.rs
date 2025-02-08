#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct FactoryVstsConfiguration {
    /// Specifies the VSTS account name.
    #[builder(into)]
    #[serde(rename = "accountName")]
    pub r#account_name: Box<String>,
    /// Specifies the branch of the repository to get code from.
    #[builder(into)]
    #[serde(rename = "branchName")]
    pub r#branch_name: Box<String>,
    /// Specifies the name of the VSTS project.
    #[builder(into)]
    #[serde(rename = "projectName")]
    pub r#project_name: Box<String>,
    /// Is automated publishing enabled? Defaults to `true`.
    #[builder(into, default)]
    #[serde(rename = "publishingEnabled")]
    pub r#publishing_enabled: Box<Option<bool>>,
    /// Specifies the name of the git repository.
    #[builder(into)]
    #[serde(rename = "repositoryName")]
    pub r#repository_name: Box<String>,
    /// Specifies the root folder within the repository. Set to `/` for the top level.
    #[builder(into)]
    #[serde(rename = "rootFolder")]
    pub r#root_folder: Box<String>,
    /// Specifies the Tenant ID associated with the VSTS account.
    #[builder(into)]
    #[serde(rename = "tenantId")]
    pub r#tenant_id: Box<String>,
}
