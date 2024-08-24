#[derive(serde::Serialize)]
pub struct ContainerCapabilities {
    /// List of linux capabilities to add.
    #[serde(rename = "adds")]
    pub r#adds: Box<Option<Vec<String>>>,
    /// List of linux capabilities to drop.
    #[serde(rename = "drops")]
    pub r#drops: Box<Option<Vec<String>>>,
}
