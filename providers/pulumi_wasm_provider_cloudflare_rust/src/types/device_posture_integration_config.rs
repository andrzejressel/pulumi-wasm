#[derive(serde::Serialize)]
pub struct DevicePostureIntegrationConfig {
    #[serde(rename = "accessClientId")]
    pub r#access_client_id: Box<Option<String>>,
    #[serde(rename = "accessClientSecret")]
    pub r#access_client_secret: Box<Option<String>>,
    #[serde(rename = "apiUrl")]
    pub r#api_url: Box<Option<String>>,
    #[serde(rename = "authUrl")]
    pub r#auth_url: Box<Option<String>>,
    #[serde(rename = "clientId")]
    pub r#client_id: Box<Option<String>>,
    #[serde(rename = "clientKey")]
    pub r#client_key: Box<Option<String>>,
    #[serde(rename = "clientSecret")]
    pub r#client_secret: Box<Option<String>>,
    #[serde(rename = "customerId")]
    pub r#customer_id: Box<Option<String>>,
}
