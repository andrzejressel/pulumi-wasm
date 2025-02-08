#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetTriggerRepositoryEventConfigPush {
    /// Regex of branches to match.
    /// 
    /// The syntax of the regular expressions accepted is the syntax accepted by
    /// RE2 and described at https://github.com/google/re2/wiki/Syntax
    #[builder(into)]
    #[serde(rename = "branch")]
    pub r#branch: Box<String>,
    /// If true, only trigger a build if the revision regex does NOT match the git_ref regex.
    #[builder(into)]
    #[serde(rename = "invertRegex")]
    pub r#invert_regex: Box<bool>,
    /// Regex of tags to match.
    /// 
    /// The syntax of the regular expressions accepted is the syntax accepted by
    /// RE2 and described at https://github.com/google/re2/wiki/Syntax
    #[builder(into)]
    #[serde(rename = "tag")]
    pub r#tag: Box<String>,
}
