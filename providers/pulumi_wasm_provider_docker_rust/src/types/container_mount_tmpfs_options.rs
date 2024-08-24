#[derive(serde::Serialize)]
pub struct ContainerMountTmpfsOptions {
    /// The permission mode for the tmpfs mount in an integer.
    #[serde(rename = "mode")]
    pub r#mode: Box<Option<i32>>,
    /// The size for the tmpfs mount in bytes.
    #[serde(rename = "sizeBytes")]
    pub r#size_bytes: Box<Option<i32>>,
}
