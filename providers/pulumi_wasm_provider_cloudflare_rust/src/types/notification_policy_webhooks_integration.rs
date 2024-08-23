#[derive(serde::Serialize)]
pub struct NotificationPolicyWebhooksIntegration {
    #[serde(rename = "id")]
    pub r#id: Box<String>,
    #[serde(rename = "name")]
    pub r#name: Box<Option<String>>,
}
