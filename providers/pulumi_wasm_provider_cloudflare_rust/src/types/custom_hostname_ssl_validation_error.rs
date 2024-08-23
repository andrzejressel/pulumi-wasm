#[derive(serde::Serialize)]
pub struct CustomHostnameSslValidationError {
    #[serde(rename = "message")]
    pub r#message: Box<Option<String>>,
}
