#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct TriggerBitbucketServerTriggerConfigPullRequest {
    /// Regex of branches to match.
    /// The syntax of the regular expressions accepted is the syntax accepted by RE2 and described at https://github.com/google/re2/wiki/Syntax
    #[builder(into)]
    #[serde(rename = "branch")]
    pub r#branch: Box<String>,
    /// Configure builds to run whether a repository owner or collaborator need to comment /gcbrun.
    /// Possible values are: `COMMENTS_DISABLED`, `COMMENTS_ENABLED`, `COMMENTS_ENABLED_FOR_EXTERNAL_CONTRIBUTORS_ONLY`.
    #[builder(into, default)]
    #[serde(rename = "commentControl")]
    pub r#comment_control: Box<Option<String>>,
    /// If true, branches that do NOT match the git_ref will trigger a build.
    #[builder(into, default)]
    #[serde(rename = "invertRegex")]
    pub r#invert_regex: Box<Option<bool>>,
}
