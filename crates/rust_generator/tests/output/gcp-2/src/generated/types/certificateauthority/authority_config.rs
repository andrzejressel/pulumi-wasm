#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct AuthorityConfig {
    /// Specifies some of the values in a certificate that are related to the subject.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "subjectConfig")]
    pub r#subject_config: Box<super::super::types::certificateauthority::AuthorityConfigSubjectConfig>,
    /// When specified this provides a custom SKI to be used in the certificate. This should only be used to maintain a SKI of an existing CA originally created outside CA service, which was not generated using method (1) described in RFC 5280 section 4.2.1.2..
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "subjectKeyId")]
    pub r#subject_key_id: Box<Option<super::super::types::certificateauthority::AuthorityConfigSubjectKeyId>>,
    /// Describes how some of the technical X.509 fields in a certificate should be populated.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "x509Config")]
    pub r#x_509_config: Box<super::super::types::certificateauthority::AuthorityConfigX509Config>,
}
