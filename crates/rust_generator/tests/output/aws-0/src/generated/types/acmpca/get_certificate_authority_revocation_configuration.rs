#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetCertificateAuthorityRevocationConfiguration {
    /// Nested attribute containing configuration of the certificate revocation list (CRL), if any, maintained by the certificate authority.
    #[builder(into)]
    #[serde(rename = "crlConfigurations")]
    pub r#crl_configurations: Box<Vec<super::super::types::acmpca::GetCertificateAuthorityRevocationConfigurationCrlConfiguration>>,
    #[builder(into)]
    #[serde(rename = "ocspConfigurations")]
    pub r#ocsp_configurations: Box<Vec<super::super::types::acmpca::GetCertificateAuthorityRevocationConfigurationOcspConfiguration>>,
}
