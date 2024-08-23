#[derive(serde::Serialize)]
pub struct ServiceTaskSpecContainerSpecMountBindOptions {
    #[serde(rename = "propagation")]
    pub r#propagation: Box<Option<String>>,
}
