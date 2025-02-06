#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct TriggerBuildSourceRepoSource {
    /// Regex matching branches to build. Exactly one a of branch name, tag, or commit SHA must be provided.
    /// The syntax of the regular expressions accepted is the syntax accepted by RE2 and
    /// described at https://github.com/google/re2/wiki/Syntax
    #[builder(into, default)]
    #[serde(rename = "branchName")]
    pub r#branch_name: Box<Option<String>>,
    /// Explicit commit SHA to build. Exactly one a of branch name, tag, or commit SHA must be provided.
    #[builder(into, default)]
    #[serde(rename = "commitSha")]
    pub r#commit_sha: Box<Option<String>>,
    /// Directory, relative to the source root, in which to run the build.
    /// This must be a relative path. If a step's dir is specified and is an absolute path,
    /// this value is ignored for that step's execution.
    #[builder(into, default)]
    #[serde(rename = "dir")]
    pub r#dir: Box<Option<String>>,
    /// Only trigger a build if the revision regex does NOT match the revision regex.
    #[builder(into, default)]
    #[serde(rename = "invertRegex")]
    pub r#invert_regex: Box<Option<bool>>,
    /// ID of the project that owns the Cloud Source Repository.
    /// If omitted, the project ID requesting the build is assumed.
    #[builder(into, default)]
    #[serde(rename = "projectId")]
    pub r#project_id: Box<Option<String>>,
    /// Name of the Cloud Source Repository.
    #[builder(into)]
    #[serde(rename = "repoName")]
    pub r#repo_name: Box<String>,
    /// Substitutions to use in a triggered build. Should only be used with triggers.run
    #[builder(into, default)]
    #[serde(rename = "substitutions")]
    pub r#substitutions: Box<Option<std::collections::HashMap<String, String>>>,
    /// Regex matching tags to build. Exactly one a of branch name, tag, or commit SHA must be provided.
    /// The syntax of the regular expressions accepted is the syntax accepted by RE2 and
    /// described at https://github.com/google/re2/wiki/Syntax
    #[builder(into, default)]
    #[serde(rename = "tagName")]
    pub r#tag_name: Box<Option<String>>,
}
