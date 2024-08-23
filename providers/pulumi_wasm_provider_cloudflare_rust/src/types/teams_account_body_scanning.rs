#[derive(serde::Serialize)]
pub struct TeamsAccountBodyScanning {
    #[serde(rename = "inspectionMode")]
    pub r#inspection_mode: Box<String>,
}
