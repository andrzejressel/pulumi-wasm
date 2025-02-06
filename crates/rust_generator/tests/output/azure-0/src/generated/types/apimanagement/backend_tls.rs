#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct BackendTls {
    /// Flag indicating whether SSL certificate chain validation should be done when using self-signed certificates for the backend host.
    #[builder(into, default)]
    #[serde(rename = "validateCertificateChain")]
    pub r#validate_certificate_chain: Box<Option<bool>>,
    /// Flag indicating whether SSL certificate name validation should be done when using self-signed certificates for the backend host.
    #[builder(into, default)]
    #[serde(rename = "validateCertificateName")]
    pub r#validate_certificate_name: Box<Option<bool>>,
}
