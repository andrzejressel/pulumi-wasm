#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct CertificateAuthorityRevocationConfiguration {
    /// Nested argument containing configuration of the certificate revocation list (CRL), if any, maintained by the certificate authority. Defined below.
    #[builder(into, default)]
    #[serde(rename = "crlConfiguration")]
    pub r#crl_configuration: Box<Option<super::super::types::acmpca::CertificateAuthorityRevocationConfigurationCrlConfiguration>>,
    /// Nested argument containing configuration of
    /// the custom OCSP responder endpoint. Defined below.
    #[builder(into, default)]
    #[serde(rename = "ocspConfiguration")]
    pub r#ocsp_configuration: Box<Option<super::super::types::acmpca::CertificateAuthorityRevocationConfigurationOcspConfiguration>>,
}