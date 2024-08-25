#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct ContainerMountVolumeOptionsLabel {
    /// Name of the label
    #[serde(rename = "label")]
    pub r#label: Box<String>,
    /// Value of the label
    #[serde(rename = "value")]
    pub r#value: Box<String>,
}
