#[derive(serde::Serialize)]
pub struct TeamsAccountBodyScanning {
    /// Body scanning inspection mode. Available values: `deep`, `shallow`.
    #[serde(rename = "inspectionMode")]
    pub r#inspection_mode: Box<String>,
}
