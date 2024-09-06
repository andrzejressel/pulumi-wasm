#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct PagesProjectDeploymentConfigsProductionPlacement {
    /// Placement Mode for the Pages Function.
    #[serde(rename = "mode")]
    pub r#mode: Box<Option<String>>,
}
