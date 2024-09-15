#[derive(serde::Deserialize, serde::Serialize, bon::Builder, Debug)]
#[builder(finish_fn = build_struct)]
pub struct RemoteImageBuildAuthConfig {
    /// the auth token
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "auth")]
    pub r#auth: Box<Option<String>>,
    /// the user emal
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "email")]
    pub r#email: Box<Option<String>>,
    /// hostname of the registry
    #[builder(into)]
    #[serde(rename = "hostName")]
    pub r#host_name: Box<String>,
    /// the identity token
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "identityToken")]
    pub r#identity_token: Box<Option<String>>,
    /// the registry password
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "password")]
    pub r#password: Box<Option<String>>,
    /// the registry token
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "registryToken")]
    pub r#registry_token: Box<Option<String>>,
    /// the server address
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "serverAddress")]
    pub r#server_address: Box<Option<String>>,
    /// the registry user name
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "userName")]
    pub r#user_name: Box<Option<String>>,
}
