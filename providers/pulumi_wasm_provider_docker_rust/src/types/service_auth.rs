#[derive(serde::Serialize)]
pub struct ServiceAuth {
    /// The password
    #[serde(rename = "password")]
    pub r#password: Box<Option<String>>,
    /// The address of the server for the authentication
    #[serde(rename = "serverAddress")]
    pub r#server_address: Box<String>,
    /// The username
    #[serde(rename = "username")]
    pub r#username: Box<Option<String>>,
}
