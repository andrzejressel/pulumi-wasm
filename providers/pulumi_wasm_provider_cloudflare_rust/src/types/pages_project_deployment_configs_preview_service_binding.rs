#[derive(serde::Serialize)]
pub struct PagesProjectDeploymentConfigsPreviewServiceBinding {
    #[serde(rename = "environment")]
    pub r#environment: Box<Option<String>>,
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    #[serde(rename = "service")]
    pub r#service: Box<String>,
}