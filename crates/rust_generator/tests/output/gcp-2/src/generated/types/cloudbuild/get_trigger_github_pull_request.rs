#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetTriggerGithubPullRequest {
    /// Regex of branches to match.
    #[builder(into)]
    #[serde(rename = "branch")]
    pub r#branch: Box<String>,
    /// Whether to block builds on a "/gcbrun" comment from a repository owner or collaborator. Possible values: ["COMMENTS_DISABLED", "COMMENTS_ENABLED", "COMMENTS_ENABLED_FOR_EXTERNAL_CONTRIBUTORS_ONLY"]
    #[builder(into)]
    #[serde(rename = "commentControl")]
    pub r#comment_control: Box<String>,
    /// If true, branches that do NOT match the git_ref will trigger a build.
    #[builder(into)]
    #[serde(rename = "invertRegex")]
    pub r#invert_regex: Box<bool>,
}
