#[derive(serde::Deserialize, serde::Serialize, bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
pub struct TeamsRuleRuleSettingsNotificationSettings {
    /// Enable notification settings.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "enabled")]
    pub r#enabled: Box<Option<bool>>,
    /// Notification content.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "message")]
    pub r#message: Box<Option<String>>,
    /// Support URL to show in the notification.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "supportUrl")]
    pub r#support_url: Box<Option<String>>,
}
