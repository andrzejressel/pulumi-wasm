#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct AppAutoBranchCreationConfig {
    /// Basic authorization credentials for the autocreated branch.
    #[builder(into, default)]
    #[serde(rename = "basicAuthCredentials")]
    pub r#basic_auth_credentials: Box<Option<String>>,
    /// Build specification (build spec) for the autocreated branch.
    #[builder(into, default)]
    #[serde(rename = "buildSpec")]
    pub r#build_spec: Box<Option<String>>,
    /// Enables auto building for the autocreated branch.
    #[builder(into, default)]
    #[serde(rename = "enableAutoBuild")]
    pub r#enable_auto_build: Box<Option<bool>>,
    /// Enables basic authorization for the autocreated branch.
    #[builder(into, default)]
    #[serde(rename = "enableBasicAuth")]
    pub r#enable_basic_auth: Box<Option<bool>>,
    /// Enables performance mode for the branch.
    #[builder(into, default)]
    #[serde(rename = "enablePerformanceMode")]
    pub r#enable_performance_mode: Box<Option<bool>>,
    /// Enables pull request previews for the autocreated branch.
    #[builder(into, default)]
    #[serde(rename = "enablePullRequestPreview")]
    pub r#enable_pull_request_preview: Box<Option<bool>>,
    /// Environment variables for the autocreated branch.
    #[builder(into, default)]
    #[serde(rename = "environmentVariables")]
    pub r#environment_variables: Box<Option<std::collections::HashMap<String, String>>>,
    /// Framework for the autocreated branch.
    #[builder(into, default)]
    #[serde(rename = "framework")]
    pub r#framework: Box<Option<String>>,
    /// Amplify environment name for the pull request.
    #[builder(into, default)]
    #[serde(rename = "pullRequestEnvironmentName")]
    pub r#pull_request_environment_name: Box<Option<String>>,
    /// Describes the current stage for the autocreated branch. Valid values: `PRODUCTION`, `BETA`, `DEVELOPMENT`, `EXPERIMENTAL`, `PULL_REQUEST`.
    #[builder(into, default)]
    #[serde(rename = "stage")]
    pub r#stage: Box<Option<String>>,
}
