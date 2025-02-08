#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct GetTriggerGithubPush {
    /// Regex of branches to match.  Specify only one of branch or tag.
    #[builder(into)]
    #[serde(rename = "branch")]
    pub r#branch: Box<String>,
    /// When true, only trigger a build if the revision regex does NOT match the git_ref regex.
    #[builder(into)]
    #[serde(rename = "invertRegex")]
    pub r#invert_regex: Box<bool>,
    /// Regex of tags to match.  Specify only one of branch or tag.
    #[builder(into)]
    #[serde(rename = "tag")]
    pub r#tag: Box<String>,
}
