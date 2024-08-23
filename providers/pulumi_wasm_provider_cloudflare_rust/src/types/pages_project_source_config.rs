#[derive(serde::Serialize)]
pub struct PagesProjectSourceConfig {
    #[serde(rename = "deploymentsEnabled")]
    pub r#deployments_enabled: Box<Option<bool>>,
    #[serde(rename = "owner")]
    pub r#owner: Box<Option<String>>,
    #[serde(rename = "prCommentsEnabled")]
    pub r#pr_comments_enabled: Box<Option<bool>>,
    #[serde(rename = "previewBranchExcludes")]
    pub r#preview_branch_excludes: Box<Option<Vec<String>>>,
    #[serde(rename = "previewBranchIncludes")]
    pub r#preview_branch_includes: Box<Option<Vec<String>>>,
    #[serde(rename = "previewDeploymentSetting")]
    pub r#preview_deployment_setting: Box<Option<String>>,
    #[serde(rename = "productionBranch")]
    pub r#production_branch: Box<String>,
    #[serde(rename = "productionDeploymentEnabled")]
    pub r#production_deployment_enabled: Box<Option<bool>>,
    #[serde(rename = "repoName")]
    pub r#repo_name: Box<Option<String>>,
}
