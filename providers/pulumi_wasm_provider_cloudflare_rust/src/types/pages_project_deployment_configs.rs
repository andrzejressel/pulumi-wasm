#[derive(serde::Serialize)]
pub struct PagesProjectDeploymentConfigs {
    #[serde(rename = "preview")]
    pub r#preview: Box<Option<crate::types::PagesProjectDeploymentConfigsPreview>>,
    #[serde(rename = "production")]
    pub r#production: Box<Option<crate::types::PagesProjectDeploymentConfigsProduction>>,
}
