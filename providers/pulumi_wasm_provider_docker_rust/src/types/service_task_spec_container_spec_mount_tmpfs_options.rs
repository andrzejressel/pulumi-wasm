#[derive(serde::Serialize)]
pub struct ServiceTaskSpecContainerSpecMountTmpfsOptions {
    #[serde(rename = "mode")]
    pub r#mode: Box<Option<i32>>,
    #[serde(rename = "sizeBytes")]
    pub r#size_bytes: Box<Option<i32>>,
}