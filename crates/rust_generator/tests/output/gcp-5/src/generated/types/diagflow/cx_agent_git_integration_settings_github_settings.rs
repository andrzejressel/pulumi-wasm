#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct CxAgentGitIntegrationSettingsGithubSettings {
    /// The access token used to authenticate the access to the GitHub repository.
    /// **Note**: This property is sensitive and will not be displayed in the plan.
    #[builder(into, default)]
    #[serde(rename = "accessToken")]
    pub r#access_token: Box<Option<String>>,
    /// A list of branches configured to be used from Dialogflow.
    #[builder(into, default)]
    #[serde(rename = "branches")]
    pub r#branches: Box<Option<Vec<String>>>,
    /// The unique repository display name for the GitHub repository.
    #[builder(into, default)]
    #[serde(rename = "displayName")]
    pub r#display_name: Box<Option<String>>,
    /// The GitHub repository URI related to the agent.
    #[builder(into, default)]
    #[serde(rename = "repositoryUri")]
    pub r#repository_uri: Box<Option<String>>,
    /// The branch of the GitHub repository tracked for this agent.
    #[builder(into, default)]
    #[serde(rename = "trackingBranch")]
    pub r#tracking_branch: Box<Option<String>>,
}
