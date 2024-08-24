#[derive(serde::Serialize)]
pub struct NotificationPolicyWebhooksIntegration {
    /// The ID of this resource.
    #[serde(rename = "id")]
    pub r#id: Box<String>,
    #[serde(rename = "name")]
    pub r#name: Box<Option<String>>,
}
