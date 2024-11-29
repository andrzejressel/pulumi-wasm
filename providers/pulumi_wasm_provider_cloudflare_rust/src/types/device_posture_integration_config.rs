#[derive(serde::Deserialize, serde::Serialize, bon::Builder, Debug)]
#[builder(finish_fn = build_struct)]
pub struct DevicePostureIntegrationConfig {
    /// The Access client ID to be used as the `Cf-Access-Client-ID` header when making a request to the `api_url`.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "accessClientId")]
    pub r#access_client_id: Box<Option<String>>,
    /// The Access client secret to be used as the `Cf-Access-Client-Secret` header when making a request to the `api_url`.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "accessClientSecret")]
    pub r#access_client_secret: Box<Option<String>>,
    /// The third-party API's URL.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "apiUrl")]
    pub r#api_url: Box<Option<String>>,
    /// The third-party authorization API URL.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "authUrl")]
    pub r#auth_url: Box<Option<String>>,
    /// The client identifier for authenticating API calls.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "clientId")]
    pub r#client_id: Box<Option<String>>,
    /// The client key for authenticating API calls.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "clientKey")]
    pub r#client_key: Box<Option<String>>,
    /// The client secret for authenticating API calls.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "clientSecret")]
    pub r#client_secret: Box<Option<String>>,
    /// The customer identifier for authenticating API calls.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "customerId")]
    pub r#customer_id: Box<Option<String>>,
}
