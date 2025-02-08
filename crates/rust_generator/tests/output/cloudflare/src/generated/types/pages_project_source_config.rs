#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct PagesProjectSourceConfig {
    /// Toggle deployments on this repo. Defaults to `true`.
    #[builder(into, default)]
    #[serde(rename = "deploymentsEnabled")]
    pub r#deployments_enabled: Box<Option<bool>>,
    /// Project owner username. **Modifying this attribute will force creation of a new resource.**
    #[builder(into, default)]
    #[serde(rename = "owner")]
    pub r#owner: Box<Option<String>>,
    /// Enable Pages to comment on Pull Requests. Defaults to `true`.
    #[builder(into, default)]
    #[serde(rename = "prCommentsEnabled")]
    pub r#pr_comments_enabled: Box<Option<bool>>,
    /// Branches will be excluded from automatic deployment.
    #[builder(into, default)]
    #[serde(rename = "previewBranchExcludes")]
    pub r#preview_branch_excludes: Box<Option<Vec<String>>>,
    /// Branches will be included for automatic deployment.
    #[builder(into, default)]
    #[serde(rename = "previewBranchIncludes")]
    pub r#preview_branch_includes: Box<Option<Vec<String>>>,
    /// Preview Deployment Setting. Available values: `custom`, `all`, `none`. Defaults to `all`.
    #[builder(into, default)]
    #[serde(rename = "previewDeploymentSetting")]
    pub r#preview_deployment_setting: Box<Option<String>>,
    /// Project production branch name.
    #[builder(into)]
    #[serde(rename = "productionBranch")]
    pub r#production_branch: Box<String>,
    /// Enable production deployments. Defaults to `true`.
    #[builder(into, default)]
    #[serde(rename = "productionDeploymentEnabled")]
    pub r#production_deployment_enabled: Box<Option<bool>>,
    /// Project repository name. **Modifying this attribute will force creation of a new resource.**
    #[builder(into, default)]
    #[serde(rename = "repoName")]
    pub r#repo_name: Box<Option<String>>,
}
