#[derive(serde::Serialize)]
pub struct ServiceAuth {
    #[serde(rename = "password")]
    pub r#password: Box<Option<String>>,
    #[serde(rename = "serverAddress")]
    pub r#server_address: Box<String>,
    #[serde(rename = "username")]
    pub r#username: Box<Option<String>>,
}
