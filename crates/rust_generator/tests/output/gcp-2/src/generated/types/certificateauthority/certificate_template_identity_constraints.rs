#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct CertificateTemplateIdentityConstraints {
    /// Required. If this is true, the SubjectAltNames extension may be copied from a certificate request into the signed certificate. Otherwise, the requested SubjectAltNames will be discarded.
    #[builder(into)]
    #[serde(rename = "allowSubjectAltNamesPassthrough")]
    pub r#allow_subject_alt_names_passthrough: Box<bool>,
    /// Required. If this is true, the Subject field may be copied from a certificate request into the signed certificate. Otherwise, the requested Subject will be discarded.
    #[builder(into)]
    #[serde(rename = "allowSubjectPassthrough")]
    pub r#allow_subject_passthrough: Box<bool>,
    /// Optional. A CEL expression that may be used to validate the resolved X.509 Subject and/or Subject Alternative Name before a certificate is signed. To see the full allowed syntax and some examples, see https://cloud.google.com/certificate-authority-service/docs/using-cel
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "celExpression")]
    pub r#cel_expression: Box<Option<super::super::types::certificateauthority::CertificateTemplateIdentityConstraintsCelExpression>>,
}
