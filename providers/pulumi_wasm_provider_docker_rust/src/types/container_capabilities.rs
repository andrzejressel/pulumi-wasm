#[derive(serde::Serialize)]
pub struct ContainerCapabilities {
    #[serde(rename = "adds")]
    pub r#adds: Box<Option<Vec<String>>>,
    #[serde(rename = "drops")]
    pub r#drops: Box<Option<Vec<String>>>,
}
