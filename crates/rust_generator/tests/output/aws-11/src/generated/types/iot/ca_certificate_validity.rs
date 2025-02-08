#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct CaCertificateValidity {
    /// The certificate is not valid after this date.
    #[builder(into, default)]
    #[serde(rename = "notAfter")]
    pub r#not_after: Box<Option<String>>,
    /// The certificate is not valid before this date.
    #[builder(into, default)]
    #[serde(rename = "notBefore")]
    pub r#not_before: Box<Option<String>>,
}
