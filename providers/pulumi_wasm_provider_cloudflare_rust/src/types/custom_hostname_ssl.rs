#[derive(serde::Deserialize, serde::Serialize, bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
pub struct CustomHostnameSsl {
    /// A ubiquitous bundle has the highest probability of being verified everywhere, even by clients using outdated or unusual trust stores. An optimal bundle uses the shortest chain and newest intermediates. And the force bundle verifies the chain, but does not otherwise modify it. Available values: `ubiquitous`, `optimal`, `force`.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "bundleMethod")]
    pub r#bundle_method: Box<Option<String>>,
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "certificateAuthority")]
    pub r#certificate_authority: Box<Option<String>>,
    /// If a custom uploaded certificate is used.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "customCertificate")]
    pub r#custom_certificate: Box<Option<String>>,
    /// The key for a custom uploaded certificate.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "customKey")]
    pub r#custom_key: Box<Option<String>>,
    /// Domain control validation (DCV) method used for this hostname. Available values: `http`, `txt`, `email`.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "method")]
    pub r#method: Box<Option<String>>,
    /// SSL/TLS settings for the certificate.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "settings")]
    pub r#settings: Box<Option<Vec<crate::types::CustomHostnameSslSetting>>>,
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "status")]
    pub r#status: Box<Option<String>>,
    /// Level of validation to be used for this hostname. Available values: `dv`. Defaults to `dv`.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "type")]
    pub r#type: Box<Option<String>>,
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "validationErrors")]
    pub r#validation_errors: Box<Option<Vec<crate::types::CustomHostnameSslValidationError>>>,
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "validationRecords")]
    pub r#validation_records: Box<Option<Vec<crate::types::CustomHostnameSslValidationRecord>>>,
    /// Indicates whether the certificate covers a wildcard.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "wildcard")]
    pub r#wildcard: Box<Option<bool>>,
}
