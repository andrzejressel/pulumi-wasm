#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct FunctionBuildConfigSourceRepoSource {
    /// Regex matching branches to build.
    #[builder(into, default)]
    #[serde(rename = "branchName")]
    pub r#branch_name: Box<Option<String>>,
    /// Regex matching tags to build.
    #[builder(into, default)]
    #[serde(rename = "commitSha")]
    pub r#commit_sha: Box<Option<String>>,
    /// Directory, relative to the source root, in which to run the build.
    #[builder(into, default)]
    #[serde(rename = "dir")]
    pub r#dir: Box<Option<String>>,
    /// Only trigger a build if the revision regex does
    /// NOT match the revision regex.
    #[builder(into, default)]
    #[serde(rename = "invertRegex")]
    pub r#invert_regex: Box<Option<bool>>,
    /// ID of the project that owns the Cloud Source Repository. If omitted, the
    /// project ID requesting the build is assumed.
    #[builder(into, default)]
    #[serde(rename = "projectId")]
    pub r#project_id: Box<Option<String>>,
    /// Name of the Cloud Source Repository.
    #[builder(into, default)]
    #[serde(rename = "repoName")]
    pub r#repo_name: Box<Option<String>>,
    /// Regex matching tags to build.
    #[builder(into, default)]
    #[serde(rename = "tagName")]
    pub r#tag_name: Box<Option<String>>,
}
