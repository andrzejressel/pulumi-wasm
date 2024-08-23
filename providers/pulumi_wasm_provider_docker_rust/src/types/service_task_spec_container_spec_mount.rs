#[derive(serde::Serialize)]
pub struct ServiceTaskSpecContainerSpecMount {
    #[serde(rename = "bindOptions")]
    pub r#bind_options: Box<Option<crate::types::ServiceTaskSpecContainerSpecMountBindOptions>>,
    #[serde(rename = "readOnly")]
    pub r#read_only: Box<Option<bool>>,
    #[serde(rename = "source")]
    pub r#source: Box<Option<String>>,
    #[serde(rename = "target")]
    pub r#target: Box<String>,
    #[serde(rename = "tmpfsOptions")]
    pub r#tmpfs_options: Box<Option<crate::types::ServiceTaskSpecContainerSpecMountTmpfsOptions>>,
    #[serde(rename = "type")]
    pub r#type: Box<String>,
    #[serde(rename = "volumeOptions")]
    pub r#volume_options: Box<Option<crate::types::ServiceTaskSpecContainerSpecMountVolumeOptions>>,
}
