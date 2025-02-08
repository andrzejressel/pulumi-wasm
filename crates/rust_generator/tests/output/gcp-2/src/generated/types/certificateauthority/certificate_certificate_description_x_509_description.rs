#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct CertificateCertificateDescriptionX509Description {
    /// (Output)
    /// Describes custom X.509 extensions.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "additionalExtensions")]
    pub r#additional_extensions: Box<Option<Vec<super::super::types::certificateauthority::CertificateCertificateDescriptionX509DescriptionAdditionalExtension>>>,
    /// (Output)
    /// Describes Online Certificate Status Protocol (OCSP) endpoint addresses that appear in the
    /// "Authority Information Access" extension in the certificate.
    #[builder(into, default)]
    #[serde(rename = "aiaOcspServers")]
    pub r#aia_ocsp_servers: Box<Option<Vec<String>>>,
    /// (Output)
    /// Describes values that are relevant in a CA certificate.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "caOptions")]
    pub r#ca_options: Box<Option<Vec<super::super::types::certificateauthority::CertificateCertificateDescriptionX509DescriptionCaOption>>>,
    /// (Output)
    /// Indicates the intended use for keys that correspond to a certificate.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "keyUsages")]
    pub r#key_usages: Box<Option<Vec<super::super::types::certificateauthority::CertificateCertificateDescriptionX509DescriptionKeyUsage>>>,
    /// (Output)
    /// Describes the X.509 name constraints extension.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "nameConstraints")]
    pub r#name_constraints: Box<Option<Vec<super::super::types::certificateauthority::CertificateCertificateDescriptionX509DescriptionNameConstraint>>>,
    /// (Output)
    /// Describes the X.509 certificate policy object identifiers, per https://tools.ietf.org/html/rfc5280#section-4.2.1.4.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "policyIds")]
    pub r#policy_ids: Box<Option<Vec<super::super::types::certificateauthority::CertificateCertificateDescriptionX509DescriptionPolicyId>>>,
}
