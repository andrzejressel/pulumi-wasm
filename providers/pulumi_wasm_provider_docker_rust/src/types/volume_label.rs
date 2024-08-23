#[derive(serde::Serialize)]
pub struct VolumeLabel {
    #[serde(rename = "label")]
    pub r#label: Box<String>,
    #[serde(rename = "value")]
    pub r#value: Box<String>,
}
