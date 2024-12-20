#[derive(serde::Deserialize, serde::Serialize, bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
pub struct ContainerMountVolumeOptionsLabel {
    /// Name of the label
    #[builder(into)]
    #[serde(rename = "label")]
    pub r#label: Box<String>,
    /// Value of the label
    #[builder(into)]
    #[serde(rename = "value")]
    pub r#value: Box<String>,
}
