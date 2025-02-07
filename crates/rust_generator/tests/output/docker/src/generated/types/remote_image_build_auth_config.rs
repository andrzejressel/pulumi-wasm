#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct RemoteImageBuildAuthConfig {
    /// the auth token
    #[builder(into, default)]
    #[serde(rename = "auth")]
    pub r#auth: Box<Option<String>>,
    /// the user emal
    #[builder(into, default)]
    #[serde(rename = "email")]
    pub r#email: Box<Option<String>>,
    /// hostname of the registry
    #[builder(into)]
    #[serde(rename = "hostName")]
    pub r#host_name: Box<String>,
    /// the identity token
    #[builder(into, default)]
    #[serde(rename = "identityToken")]
    pub r#identity_token: Box<Option<String>>,
    /// the registry password
    #[builder(into, default)]
    #[serde(rename = "password")]
    pub r#password: Box<Option<String>>,
    /// the registry token
    #[builder(into, default)]
    #[serde(rename = "registryToken")]
    pub r#registry_token: Box<Option<String>>,
    /// the server address
    #[builder(into, default)]
    #[serde(rename = "serverAddress")]
    pub r#server_address: Box<Option<String>>,
    /// the registry user name
    #[builder(into, default)]
    #[serde(rename = "userName")]
    pub r#user_name: Box<Option<String>>,
}
