#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct RepositoryInitialConfig {
    /// Default branch name of the repository.
    #[builder(into, default)]
    #[serde(rename = "defaultBranch")]
    pub r#default_branch: Box<Option<String>>,
    /// List of gitignore template names user can choose from.
    /// Valid values can be viewed at https://cloud.google.com/secure-source-manager/docs/reference/rest/v1/projects.locations.repositories#initialconfig.
    #[builder(into, default)]
    #[serde(rename = "gitignores")]
    pub r#gitignores: Box<Option<Vec<String>>>,
    /// License template name user can choose from.
    /// Valid values can be viewed at https://cloud.google.com/secure-source-manager/docs/reference/rest/v1/projects.locations.repositories#initialconfig.
    #[builder(into, default)]
    #[serde(rename = "license")]
    pub r#license: Box<Option<String>>,
    /// README template name.
    /// Valid values can be viewed at https://cloud.google.com/secure-source-manager/docs/reference/rest/v1/projects.locations.repositories#initialconfig.
    #[builder(into, default)]
    #[serde(rename = "readme")]
    pub r#readme: Box<Option<String>>,
}
