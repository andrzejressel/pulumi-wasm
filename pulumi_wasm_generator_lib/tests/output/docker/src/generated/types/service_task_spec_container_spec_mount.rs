#[derive(serde::Deserialize, serde::Serialize, bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
pub struct ServiceTaskSpecContainerSpecMount {
    /// Optional configuration for the bind type
    #[builder(into, default)]
    #[serde(rename = "bindOptions")]
    pub r#bind_options: Box<Option<super::types::ServiceTaskSpecContainerSpecMountBindOptions>>,
    /// Whether the mount should be read-only
    #[builder(into, default)]
    #[serde(rename = "readOnly")]
    pub r#read_only: Box<Option<bool>>,
    /// Mount source (e.g. a volume name, a host path)
    #[builder(into, default)]
    #[serde(rename = "source")]
    pub r#source: Box<Option<String>>,
    /// Container path
    #[builder(into)]
    #[serde(rename = "target")]
    pub r#target: Box<String>,
    /// Optional configuration for the tmpfs type
    #[builder(into, default)]
    #[serde(rename = "tmpfsOptions")]
    pub r#tmpfs_options: Box<Option<super::types::ServiceTaskSpecContainerSpecMountTmpfsOptions>>,
    /// The mount type
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type: Box<String>,
    /// Optional configuration for the volume type
    #[builder(into, default)]
    #[serde(rename = "volumeOptions")]
    pub r#volume_options: Box<Option<super::types::ServiceTaskSpecContainerSpecMountVolumeOptions>>,
}
