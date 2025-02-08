#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct CertificateCertificateDescriptionX509DescriptionAdditionalExtension {
    /// Indicates whether or not this extension is critical (i.e., if the client does not know how to
    /// handle this extension, the client should consider this to be an error).
    #[builder(into, default)]
    #[serde(rename = "critical")]
    pub r#critical: Box<Option<bool>>,
    /// Describes values that are relevant in a CA certificate.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "objectIds")]
    pub r#object_ids: Box<Option<Vec<super::super::types::certificateauthority::CertificateCertificateDescriptionX509DescriptionAdditionalExtensionObjectId>>>,
    /// The value of this X.509 extension. A base64-encoded string.
    #[builder(into, default)]
    #[serde(rename = "value")]
    pub r#value: Box<Option<String>>,
}
