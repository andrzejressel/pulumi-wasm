#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct CustomTargetTypeCustomActionsIncludeSkaffoldModule {
    /// The Skaffold Config modules to use from the specified source.
    #[builder(into, default)]
    #[serde(rename = "configs")]
    pub r#configs: Box<Option<Vec<String>>>,
    /// Remote git repository containing the Skaffold Config modules.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "git")]
    pub r#git: Box<Option<super::super::types::clouddeploy::CustomTargetTypeCustomActionsIncludeSkaffoldModuleGit>>,
    /// Cloud Build 2nd gen repository containing the Skaffold Config modules.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "googleCloudBuildRepo")]
    pub r#google_cloud_build_repo: Box<Option<super::super::types::clouddeploy::CustomTargetTypeCustomActionsIncludeSkaffoldModuleGoogleCloudBuildRepo>>,
    /// Cloud Storage bucket containing Skaffold Config modules.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "googleCloudStorage")]
    pub r#google_cloud_storage: Box<Option<super::super::types::clouddeploy::CustomTargetTypeCustomActionsIncludeSkaffoldModuleGoogleCloudStorage>>,
}
