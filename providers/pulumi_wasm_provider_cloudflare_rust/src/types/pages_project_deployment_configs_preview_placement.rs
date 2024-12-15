#[derive(serde::Deserialize, serde::Serialize, bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
pub struct PagesProjectDeploymentConfigsPreviewPlacement {
    /// Placement Mode for the Pages Function.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "mode")]
    pub r#mode: Box<Option<String>>,
}
