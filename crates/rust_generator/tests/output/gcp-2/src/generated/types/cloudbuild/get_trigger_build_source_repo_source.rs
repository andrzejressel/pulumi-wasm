#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetTriggerBuildSourceRepoSource {
    /// Regex matching branches to build. Exactly one a of branch name, tag, or commit SHA must be provided.
    /// The syntax of the regular expressions accepted is the syntax accepted by RE2 and
    /// described at https://github.com/google/re2/wiki/Syntax
    #[builder(into)]
    #[serde(rename = "branchName")]
    pub r#branch_name: Box<String>,
    /// Explicit commit SHA to build. Exactly one a of branch name, tag, or commit SHA must be provided.
    #[builder(into)]
    #[serde(rename = "commitSha")]
    pub r#commit_sha: Box<String>,
    /// Directory, relative to the source root, in which to run the build.
    /// This must be a relative path. If a step's dir is specified and is an absolute path,
    /// this value is ignored for that step's execution.
    #[builder(into)]
    #[serde(rename = "dir")]
    pub r#dir: Box<String>,
    /// Only trigger a build if the revision regex does NOT match the revision regex.
    #[builder(into)]
    #[serde(rename = "invertRegex")]
    pub r#invert_regex: Box<bool>,
    /// ID of the project that owns the Cloud Source Repository.
    /// If omitted, the project ID requesting the build is assumed.
    #[builder(into)]
    #[serde(rename = "projectId")]
    pub r#project_id: Box<String>,
    /// Name of the Cloud Source Repository.
    #[builder(into)]
    #[serde(rename = "repoName")]
    pub r#repo_name: Box<String>,
    /// Substitutions to use in a triggered build. Should only be used with triggers.run
    #[builder(into)]
    #[serde(rename = "substitutions")]
    pub r#substitutions: Box<std::collections::HashMap<String, String>>,
    /// Regex matching tags to build. Exactly one a of branch name, tag, or commit SHA must be provided.
    /// The syntax of the regular expressions accepted is the syntax accepted by RE2 and
    /// described at https://github.com/google/re2/wiki/Syntax
    #[builder(into)]
    #[serde(rename = "tagName")]
    pub r#tag_name: Box<String>,
}
