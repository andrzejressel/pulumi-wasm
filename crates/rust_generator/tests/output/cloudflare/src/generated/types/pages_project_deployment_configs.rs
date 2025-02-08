#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct PagesProjectDeploymentConfigs {
    /// Configuration for preview deploys.
    #[builder(into, default)]
    #[serde(rename = "preview")]
    pub r#preview: Box<Option<super::types::PagesProjectDeploymentConfigsPreview>>,
    /// Configuration for production deploys.
    #[builder(into, default)]
    #[serde(rename = "production")]
    pub r#production: Box<Option<super::types::PagesProjectDeploymentConfigsProduction>>,
}
