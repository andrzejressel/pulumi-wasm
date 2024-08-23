#[derive(serde::Serialize)]
pub struct RemoteImageBuildAuthConfig {
    #[serde(rename = "auth")]
    pub r#auth: Box<Option<String>>,
    #[serde(rename = "email")]
    pub r#email: Box<Option<String>>,
    #[serde(rename = "hostName")]
    pub r#host_name: Box<String>,
    #[serde(rename = "identityToken")]
    pub r#identity_token: Box<Option<String>>,
    #[serde(rename = "password")]
    pub r#password: Box<Option<String>>,
    #[serde(rename = "registryToken")]
    pub r#registry_token: Box<Option<String>>,
    #[serde(rename = "serverAddress")]
    pub r#server_address: Box<Option<String>>,
    #[serde(rename = "userName")]
    pub r#user_name: Box<Option<String>>,
}
