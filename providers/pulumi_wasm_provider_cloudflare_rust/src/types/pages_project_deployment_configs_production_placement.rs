#[derive(serde::Serialize)]
pub struct PagesProjectDeploymentConfigsProductionPlacement {
    /// Placement Mode for the Pages Function.
    #[serde(rename = "mode")]
    pub r#mode: Box<Option<String>>,
}
