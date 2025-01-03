#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct CertificateCertificateDescription {
    /// (Output)
    /// Describes lists of issuer CA certificate URLs that appear in the "Authority Information Access" extension in the certificate.
    #[builder(into, default)]
    #[serde(rename = "aiaIssuingCertificateUrls")]
    pub r#aia_issuing_certificate_urls: Box<Option<Vec<String>>>,
    /// (Output)
    /// Identifies the subjectKeyId of the parent certificate, per https://tools.ietf.org/html/rfc5280#section-4.2.1.1
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "authorityKeyIds")]
    pub r#authority_key_ids: Box<Option<Vec<super::super::types::certificateauthority::CertificateCertificateDescriptionAuthorityKeyId>>>,
    /// (Output)
    /// The hash of the x.509 certificate.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "certFingerprints")]
    pub r#cert_fingerprints: Box<Option<Vec<super::super::types::certificateauthority::CertificateCertificateDescriptionCertFingerprint>>>,
    /// (Output)
    /// Describes a list of locations to obtain CRL information, i.e. the DistributionPoint.fullName described by https://tools.ietf.org/html/rfc5280#section-4.2.1.13
    #[builder(into, default)]
    #[serde(rename = "crlDistributionPoints")]
    pub r#crl_distribution_points: Box<Option<Vec<String>>>,
    /// (Output)
    /// A PublicKey describes a public key.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "publicKeys")]
    pub r#public_keys: Box<Option<Vec<super::super::types::certificateauthority::CertificateCertificateDescriptionPublicKey>>>,
    /// (Output)
    /// Describes some of the values in a certificate that are related to the subject and lifetime.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "subjectDescriptions")]
    pub r#subject_descriptions: Box<Option<Vec<super::super::types::certificateauthority::CertificateCertificateDescriptionSubjectDescription>>>,
    /// (Output)
    /// Provides a means of identifiying certificates that contain a particular public key, per https://tools.ietf.org/html/rfc5280#section-4.2.1.2.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "subjectKeyIds")]
    pub r#subject_key_ids: Box<Option<Vec<super::super::types::certificateauthority::CertificateCertificateDescriptionSubjectKeyId>>>,
    /// (Output)
    /// A structured description of the issued X.509 certificate.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "x509Descriptions")]
    pub r#x_509_descriptions: Box<Option<Vec<super::super::types::certificateauthority::CertificateCertificateDescriptionX509Description>>>,
}
