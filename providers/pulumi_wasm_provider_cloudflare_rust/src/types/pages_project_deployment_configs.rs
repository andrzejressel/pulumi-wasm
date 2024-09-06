#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct PagesProjectDeploymentConfigs {
    /// Configuration for preview deploys.
    #[serde(rename = "preview")]
    pub r#preview: Box<Option<crate::types::PagesProjectDeploymentConfigsPreview>>,
    /// Configuration for production deploys.
    #[serde(rename = "production")]
    pub r#production: Box<Option<crate::types::PagesProjectDeploymentConfigsProduction>>,
}
