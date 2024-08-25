#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct ServiceTaskSpecContainerSpecMount {
    /// Optional configuration for the bind type
    #[serde(rename = "bindOptions")]
    pub r#bind_options: Box<Option<crate::types::ServiceTaskSpecContainerSpecMountBindOptions>>,
    /// Whether the mount should be read-only
    #[serde(rename = "readOnly")]
    pub r#read_only: Box<Option<bool>>,
    /// Mount source (e.g. a volume name, a host path)
    #[serde(rename = "source")]
    pub r#source: Box<Option<String>>,
    /// Container path
    #[serde(rename = "target")]
    pub r#target: Box<String>,
    /// Optional configuration for the tmpfs type
    #[serde(rename = "tmpfsOptions")]
    pub r#tmpfs_options: Box<Option<crate::types::ServiceTaskSpecContainerSpecMountTmpfsOptions>>,
    /// The mount type
    #[serde(rename = "type")]
    pub r#type: Box<String>,
    /// Optional configuration for the volume type
    #[serde(rename = "volumeOptions")]
    pub r#volume_options: Box<Option<crate::types::ServiceTaskSpecContainerSpecMountVolumeOptions>>,
}
