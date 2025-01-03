#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ProjectSource {
    /// Configuration block that contains information that defines how the build project reports the build status to the source provider. This option is only used when the source provider is GitHub, GitHub Enterprise, GitLab, GitLab Self Managed, or Bitbucket. `build_status_config` blocks are documented below.
    #[builder(into, default)]
    #[serde(rename = "buildStatusConfig")]
    pub r#build_status_config: Box<Option<super::super::types::codebuild::ProjectSourceBuildStatusConfig>>,
    /// Build specification to use for this build project's related builds. This must be set when `type` is `NO_SOURCE`. Also, if a non-default buildspec file name or file path aside from the root is used, it must be specified.
    #[builder(into, default)]
    #[serde(rename = "buildspec")]
    pub r#buildspec: Box<Option<String>>,
    /// Truncate git history to this many commits. Use `0` for a `Full` checkout which you need to run commands like `git branch --show-current`. See [AWS CodePipeline User Guide: Tutorial: Use full clone with a GitHub pipeline source](https://docs.aws.amazon.com/codepipeline/latest/userguide/tutorials-github-gitclone.html) for details.
    #[builder(into, default)]
    #[serde(rename = "gitCloneDepth")]
    pub r#git_clone_depth: Box<Option<i32>>,
    /// Configuration block. Detailed below.
    #[builder(into, default)]
    #[serde(rename = "gitSubmodulesConfig")]
    pub r#git_submodules_config: Box<Option<super::super::types::codebuild::ProjectSourceGitSubmodulesConfig>>,
    /// Ignore SSL warnings when connecting to source control.
    #[builder(into, default)]
    #[serde(rename = "insecureSsl")]
    pub r#insecure_ssl: Box<Option<bool>>,
    /// Location of the source code from git or s3.
    #[builder(into, default)]
    #[serde(rename = "location")]
    pub r#location: Box<Option<String>>,
    /// Whether to report the status of a build's start and finish to your source provider. This option is valid only when your source provider is GitHub, GitHub Enterprise, GitLab, GitLab Self Managed, or Bitbucket.
    #[builder(into, default)]
    #[serde(rename = "reportBuildStatus")]
    pub r#report_build_status: Box<Option<bool>>,
    /// Type of repository that contains the source code to be built. Valid values: `BITBUCKET`, `CODECOMMIT`, `CODEPIPELINE`, `GITHUB`, `GITHUB_ENTERPRISE`, `GITLAB`, `GITLAB_SELF_MANAGED`, `NO_SOURCE`, `S3`.
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: Box<String>,
}
