#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct CertificateCertificateDescriptionSubjectDescriptionSubjectAltNameCustomSan {
    /// (Output)
    /// Indicates whether or not the name constraints are marked critical.
    #[builder(into, default)]
    #[serde(rename = "critical")]
    pub r#critical: Box<Option<bool>>,
    /// (Output)
    /// Describes how some of the technical fields in a certificate should be populated.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "obectIds")]
    pub r#obect_ids: Box<Option<Vec<super::super::types::certificateauthority::CertificateCertificateDescriptionSubjectDescriptionSubjectAltNameCustomSanObectId>>>,
    /// The value of this X.509 extension. A base64-encoded string.
    #[builder(into, default)]
    #[serde(rename = "value")]
    pub r#value: Box<Option<String>>,
}
