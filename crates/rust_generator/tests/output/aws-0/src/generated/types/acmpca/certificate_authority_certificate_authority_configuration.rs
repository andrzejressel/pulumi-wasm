#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct CertificateAuthorityCertificateAuthorityConfiguration {
    /// Type of the public key algorithm and size, in bits, of the key pair that your key pair creates when it issues a certificate. Valid values can be found in the [ACM PCA Documentation](https://docs.aws.amazon.com/privateca/latest/APIReference/API_CertificateAuthorityConfiguration.html).
    #[builder(into)]
    #[serde(rename = "keyAlgorithm")]
    pub r#key_algorithm: Box<String>,
    /// Name of the algorithm your private CA uses to sign certificate requests. Valid values can be found in the [ACM PCA Documentation](https://docs.aws.amazon.com/privateca/latest/APIReference/API_CertificateAuthorityConfiguration.html).
    #[builder(into)]
    #[serde(rename = "signingAlgorithm")]
    pub r#signing_algorithm: Box<String>,
    /// Nested argument that contains X.500 distinguished name information. At least one nested attribute must be specified.
    #[builder(into)]
    #[serde(rename = "subject")]
    pub r#subject: Box<super::super::types::acmpca::CertificateAuthorityCertificateAuthorityConfigurationSubject>,
}
