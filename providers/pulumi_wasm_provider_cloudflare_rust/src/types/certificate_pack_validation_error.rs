#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct CertificatePackValidationError {
    #[serde(rename = "message")]
    pub r#message: Box<Option<String>>,
}
