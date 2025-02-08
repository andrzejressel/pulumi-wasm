#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct CertificateCertificateDescriptionX509DescriptionKeyUsage {
    /// Describes high-level ways in which a key may be used.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "baseKeyUsages")]
    pub r#base_key_usages: Box<Option<Vec<super::super::types::certificateauthority::CertificateCertificateDescriptionX509DescriptionKeyUsageBaseKeyUsage>>>,
    /// Describes high-level ways in which a key may be used.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "extendedKeyUsages")]
    pub r#extended_key_usages: Box<Option<Vec<super::super::types::certificateauthority::CertificateCertificateDescriptionX509DescriptionKeyUsageExtendedKeyUsage>>>,
    /// An ObjectId specifies an object identifier (OID). These provide context and describe types in ASN.1 messages.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "unknownExtendedKeyUsages")]
    pub r#unknown_extended_key_usages: Box<Option<Vec<super::super::types::certificateauthority::CertificateCertificateDescriptionX509DescriptionKeyUsageUnknownExtendedKeyUsage>>>,
}
