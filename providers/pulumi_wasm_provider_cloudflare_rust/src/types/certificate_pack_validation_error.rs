#[derive(serde::Serialize)]
pub struct CertificatePackValidationError {
    #[serde(rename = "message")]
    pub r#message: Box<Option<String>>,
}
