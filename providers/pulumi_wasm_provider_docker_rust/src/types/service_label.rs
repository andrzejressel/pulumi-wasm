#[derive(serde::Serialize)]
pub struct ServiceLabel {
    #[serde(rename = "label")]
    pub r#label: Box<String>,
    #[serde(rename = "value")]
    pub r#value: Box<String>,
}
