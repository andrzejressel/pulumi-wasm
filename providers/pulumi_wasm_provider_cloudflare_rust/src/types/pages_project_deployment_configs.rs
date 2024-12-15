#[derive(serde::Deserialize, serde::Serialize, bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
pub struct PagesProjectDeploymentConfigs {
    /// Configuration for preview deploys.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "preview")]
    pub r#preview: Box<Option<crate::types::PagesProjectDeploymentConfigsPreview>>,
    /// Configuration for production deploys.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "production")]
    pub r#production: Box<Option<crate::types::PagesProjectDeploymentConfigsProduction>>,
}
