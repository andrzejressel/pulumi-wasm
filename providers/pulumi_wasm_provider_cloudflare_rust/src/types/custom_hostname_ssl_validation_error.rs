#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct CustomHostnameSslValidationError {
    #[serde(rename = "message")]
    pub r#message: Box<Option<String>>,
}
