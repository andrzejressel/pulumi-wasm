#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct TeamsAccountBodyScanning {
    /// Body scanning inspection mode. Available values: `deep`, `shallow`.
    #[serde(rename = "inspectionMode")]
    pub r#inspection_mode: Box<String>,
}
