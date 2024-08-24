#[derive(serde::Serialize)]
pub struct RemoteImageBuildAuthConfig {
    /// the auth token
    #[serde(rename = "auth")]
    pub r#auth: Box<Option<String>>,
    /// the user emal
    #[serde(rename = "email")]
    pub r#email: Box<Option<String>>,
    /// hostname of the registry
    #[serde(rename = "hostName")]
    pub r#host_name: Box<String>,
    /// the identity token
    #[serde(rename = "identityToken")]
    pub r#identity_token: Box<Option<String>>,
    /// the registry password
    #[serde(rename = "password")]
    pub r#password: Box<Option<String>>,
    /// the registry token
    #[serde(rename = "registryToken")]
    pub r#registry_token: Box<Option<String>>,
    /// the server address
    #[serde(rename = "serverAddress")]
    pub r#server_address: Box<Option<String>>,
    /// the registry user name
    #[serde(rename = "userName")]
    pub r#user_name: Box<Option<String>>,
}
