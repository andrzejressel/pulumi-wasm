#[derive(serde::Deserialize, serde::Serialize, bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
pub struct ContainerMountBindOptions {
    /// A propagation mode with the value.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "propagation")]
    pub r#propagation: Box<Option<String>>,
}
