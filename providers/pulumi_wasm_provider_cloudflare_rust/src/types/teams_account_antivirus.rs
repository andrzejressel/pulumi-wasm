#[derive(serde::Serialize)]
pub struct TeamsAccountAntivirus {
    #[serde(rename = "enabledDownloadPhase")]
    pub r#enabled_download_phase: Box<bool>,
    #[serde(rename = "enabledUploadPhase")]
    pub r#enabled_upload_phase: Box<bool>,
    #[serde(rename = "failClosed")]
    pub r#fail_closed: Box<bool>,
    #[serde(rename = "notificationSettings")]
    pub r#notification_settings:
        Box<Option<crate::types::TeamsAccountAntivirusNotificationSettings>>,
}
