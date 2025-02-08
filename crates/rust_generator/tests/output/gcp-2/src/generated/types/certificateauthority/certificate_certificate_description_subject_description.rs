#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct CertificateCertificateDescriptionSubjectDescription {
    /// (Output)
    /// The serial number encoded in lowercase hexadecimal.
    #[builder(into, default)]
    #[serde(rename = "hexSerialNumber")]
    pub r#hex_serial_number: Box<Option<String>>,
    /// The desired lifetime of the CA certificate. Used to create the "notBeforeTime" and
    /// "notAfterTime" fields inside an X.509 certificate. A duration in seconds with up to nine
    /// fractional digits, terminated by 's'. Example: "3.5s".
    #[builder(into, default)]
    #[serde(rename = "lifetime")]
    pub r#lifetime: Box<Option<String>>,
    /// (Output)
    /// The time at which the certificate expires.
    #[builder(into, default)]
    #[serde(rename = "notAfterTime")]
    pub r#not_after_time: Box<Option<String>>,
    /// (Output)
    /// The time at which the certificate becomes valid.
    #[builder(into, default)]
    #[serde(rename = "notBeforeTime")]
    pub r#not_before_time: Box<Option<String>>,
    /// (Output)
    /// The subject alternative name fields.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "subjectAltNames")]
    pub r#subject_alt_names: Box<Option<Vec<super::super::types::certificateauthority::CertificateCertificateDescriptionSubjectDescriptionSubjectAltName>>>,
    /// (Output)
    /// Contains distinguished name fields such as the location and organization.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "subjects")]
    pub r#subjects: Box<Option<Vec<super::super::types::certificateauthority::CertificateCertificateDescriptionSubjectDescriptionSubject>>>,
}
