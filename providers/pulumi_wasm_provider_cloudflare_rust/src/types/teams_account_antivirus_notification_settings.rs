#[derive(serde::Serialize)]
pub struct TeamsAccountAntivirusNotificationSettings {
    /// Enable notification settings.
    #[serde(rename = "enabled")]
    pub r#enabled: Box<Option<bool>>,
    /// Notification content.
    #[serde(rename = "message")]
    pub r#message: Box<Option<String>>,
    /// Support URL to show in the notification.
    #[serde(rename = "supportUrl")]
    pub r#support_url: Box<Option<String>>,
}
