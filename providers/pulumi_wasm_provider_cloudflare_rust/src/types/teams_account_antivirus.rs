#[derive(serde::Deserialize, serde::Serialize, bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
pub struct TeamsAccountAntivirus {
    /// Scan on file download.
    #[builder(into)]
    #[serde(rename = "enabledDownloadPhase")]
    pub r#enabled_download_phase: Box<bool>,
    /// Scan on file upload.
    #[builder(into)]
    #[serde(rename = "enabledUploadPhase")]
    pub r#enabled_upload_phase: Box<bool>,
    /// Block requests for files that cannot be scanned.
    #[builder(into)]
    #[serde(rename = "failClosed")]
    pub r#fail_closed: Box<bool>,
    /// Set notifications for antivirus.
    #[builder(into, default)]
    #[serde(rename = "notificationSettings")]
    pub r#notification_settings: Box<Option<crate::types::TeamsAccountAntivirusNotificationSettings>>,
}
