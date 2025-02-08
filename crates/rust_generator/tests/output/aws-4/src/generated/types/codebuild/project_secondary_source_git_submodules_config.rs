#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct ProjectSecondarySourceGitSubmodulesConfig {
    /// Whether to fetch Git submodules for the AWS CodeBuild build project.
    #[builder(into)]
    #[serde(rename = "fetchSubmodules")]
    pub r#fetch_submodules: Box<bool>,
}
