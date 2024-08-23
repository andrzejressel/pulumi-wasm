#[derive(serde::Serialize)]
pub struct Registry {
    #[serde(rename = "password")]
    pub r#password: Box<Option<String>>,
    #[serde(rename = "server")]
    pub r#server: Box<Option<String>>,
    #[serde(rename = "username")]
    pub r#username: Box<Option<String>>,
}
