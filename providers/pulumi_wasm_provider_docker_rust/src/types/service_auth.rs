#[derive(serde::Deserialize, serde::Serialize, bon::Builder, Debug)]
#[builder(finish_fn = build_struct)]
pub struct ServiceAuth {
    /// The password
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "password")]
    pub r#password: Box<Option<String>>,
    /// The address of the server for the authentication
    #[builder(into)]
    #[serde(rename = "serverAddress")]
    pub r#server_address: Box<String>,
    /// The username
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "username")]
    pub r#username: Box<Option<String>>,
}
