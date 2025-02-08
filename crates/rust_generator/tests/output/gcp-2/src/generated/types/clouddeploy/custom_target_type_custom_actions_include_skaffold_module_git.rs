#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct CustomTargetTypeCustomActionsIncludeSkaffoldModuleGit {
    /// Relative path from the repository root to the Skaffold file.
    #[builder(into, default)]
    #[serde(rename = "path")]
    pub r#path: Box<Option<String>>,
    /// Git ref the package should be cloned from.
    #[builder(into, default)]
    #[serde(rename = "ref")]
    pub r#ref_: Box<Option<String>>,
    /// Git repository the package should be cloned from.
    #[builder(into)]
    #[serde(rename = "repo")]
    pub r#repo: Box<String>,
}
