#[derive(serde::Serialize)]
pub struct TeamsAccountAntivirus {
    /// Scan on file download.
    #[serde(rename = "enabledDownloadPhase")]
    pub r#enabled_download_phase: Box<bool>,
    /// Scan on file upload.
    #[serde(rename = "enabledUploadPhase")]
    pub r#enabled_upload_phase: Box<bool>,
    /// Block requests for files that cannot be scanned.
    #[serde(rename = "failClosed")]
    pub r#fail_closed: Box<bool>,
    /// Set notifications for antivirus.
    #[serde(rename = "notificationSettings")]
    pub r#notification_settings:
        Box<Option<crate::types::TeamsAccountAntivirusNotificationSettings>>,
}
