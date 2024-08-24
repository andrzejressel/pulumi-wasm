#[derive(serde::Serialize)]
pub struct PagesProjectDeploymentConfigsPreviewPlacement {
    /// Placement Mode for the Pages Function.
    #[serde(rename = "mode")]
    pub r#mode: Box<Option<String>>,
}
