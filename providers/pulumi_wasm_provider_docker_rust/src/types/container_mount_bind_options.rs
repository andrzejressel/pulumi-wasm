#[derive(serde::Serialize)]
pub struct ContainerMountBindOptions {
    /// A propagation mode with the value.
    #[serde(rename = "propagation")]
    pub r#propagation: Box<Option<String>>,
}
