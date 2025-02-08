#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct CertificateCertificatePolicy {
    /// A `issuer_parameters` block as defined below.
    #[builder(into)]
    #[serde(rename = "issuerParameters")]
    pub r#issuer_parameters: Box<super::super::types::keyvault::CertificateCertificatePolicyIssuerParameters>,
    /// A `key_properties` block as defined below.
    #[builder(into)]
    #[serde(rename = "keyProperties")]
    pub r#key_properties: Box<super::super::types::keyvault::CertificateCertificatePolicyKeyProperties>,
    /// A `lifetime_action` block as defined below.
    #[builder(into, default)]
    #[serde(rename = "lifetimeActions")]
    pub r#lifetime_actions: Box<Option<Vec<super::super::types::keyvault::CertificateCertificatePolicyLifetimeAction>>>,
    /// A `secret_properties` block as defined below.
    #[builder(into)]
    #[serde(rename = "secretProperties")]
    pub r#secret_properties: Box<super::super::types::keyvault::CertificateCertificatePolicySecretProperties>,
    /// A `x509_certificate_properties` block as defined below. Required when `certificate` block is not specified.
    #[builder(into, default)]
    #[serde(rename = "x509CertificateProperties")]
    pub r#x_509_certificate_properties: Box<Option<super::super::types::keyvault::CertificateCertificatePolicyX509CertificateProperties>>,
}
