#[derive(serde::Serialize)]
pub struct ServiceLabel {
    /// Name of the label
    #[serde(rename = "label")]
    pub r#label: Box<String>,
    /// Value of the label
    #[serde(rename = "value")]
    pub r#value: Box<String>,
}
