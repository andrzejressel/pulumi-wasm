#[derive(serde::Serialize)]
pub struct CustomHostnameSsl {
    #[serde(rename = "bundleMethod")]
    pub r#bundle_method: Box<Option<String>>,
    #[serde(rename = "certificateAuthority")]
    pub r#certificate_authority: Box<Option<String>>,
    #[serde(rename = "customCertificate")]
    pub r#custom_certificate: Box<Option<String>>,
    #[serde(rename = "customKey")]
    pub r#custom_key: Box<Option<String>>,
    #[serde(rename = "method")]
    pub r#method: Box<Option<String>>,
    #[serde(rename = "settings")]
    pub r#settings: Box<Option<Vec<crate::types::CustomHostnameSslSetting>>>,
    #[serde(rename = "status")]
    pub r#status: Box<Option<String>>,
    #[serde(rename = "type")]
    pub r#type: Box<Option<String>>,
    #[serde(rename = "validationErrors")]
    pub r#validation_errors: Box<Option<Vec<crate::types::CustomHostnameSslValidationError>>>,
    #[serde(rename = "validationRecords")]
    pub r#validation_records: Box<Option<Vec<crate::types::CustomHostnameSslValidationRecord>>>,
    #[serde(rename = "wildcard")]
    pub r#wildcard: Box<Option<bool>>,
}
