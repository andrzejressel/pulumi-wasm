#[derive(serde::Serialize)]
pub struct TeamsAccountAntivirusNotificationSettings {
    #[serde(rename = "enabled")]
    pub r#enabled: Box<Option<bool>>,
    #[serde(rename = "message")]
    pub r#message: Box<Option<String>>,
    #[serde(rename = "supportUrl")]
    pub r#support_url: Box<Option<String>>,
}
