#[derive(serde::Deserialize, serde::Serialize, bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
pub struct ContainerCapabilities {
    /// List of linux capabilities to add.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "adds")]
    pub r#adds: Box<Option<Vec<String>>>,
    /// List of linux capabilities to drop.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "drops")]
    pub r#drops: Box<Option<Vec<String>>>,
}
