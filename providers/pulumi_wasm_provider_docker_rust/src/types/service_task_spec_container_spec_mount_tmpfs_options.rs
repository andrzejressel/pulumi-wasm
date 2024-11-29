#[derive(serde::Deserialize, serde::Serialize, bon::Builder, Debug)]
#[builder(finish_fn = build_struct)]
pub struct ServiceTaskSpecContainerSpecMountTmpfsOptions {
    /// The permission mode for the tmpfs mount in an integer
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "mode")]
    pub r#mode: Box<Option<i32>>,
    /// The size for the tmpfs mount in bytes
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "sizeBytes")]
    pub r#size_bytes: Box<Option<i32>>,
}
