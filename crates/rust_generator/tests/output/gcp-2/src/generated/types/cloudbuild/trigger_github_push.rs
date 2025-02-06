#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct TriggerGithubPush {
    /// Regex of branches to match.  Specify only one of branch or tag.
    #[builder(into, default)]
    #[serde(rename = "branch")]
    pub r#branch: Box<Option<String>>,
    /// When true, only trigger a build if the revision regex does NOT match the gitRef regex.
    #[builder(into, default)]
    #[serde(rename = "invertRegex")]
    pub r#invert_regex: Box<Option<bool>>,
    /// Regex of tags to match.  Specify only one of branch or tag.
    #[builder(into, default)]
    #[serde(rename = "tag")]
    pub r#tag: Box<Option<String>>,
}
