#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct NotificationPolicyEmailIntegration {
    /// The ID of this resource.
    #[serde(rename = "id")]
    pub r#id: Box<String>,
    #[serde(rename = "name")]
    pub r#name: Box<Option<String>>,
}
