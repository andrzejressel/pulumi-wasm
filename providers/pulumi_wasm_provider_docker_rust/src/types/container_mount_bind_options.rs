#[derive(serde::Serialize)]
pub struct ContainerMountBindOptions {
    #[serde(rename = "propagation")]
    pub r#propagation: Box<Option<String>>,
}
