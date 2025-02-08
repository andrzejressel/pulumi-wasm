#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct CertificateTemplatePredefinedValuesCaOptions {
    /// Optional. Refers to the "CA" X.509 extension, which is a boolean value. When this value is missing, the extension will be omitted from the CA certificate.
    #[builder(into, default)]
    #[serde(rename = "isCa")]
    pub r#is_ca: Box<Option<bool>>,
    /// Optional. Refers to the path length restriction X.509 extension. For a CA certificate, this value describes the depth of subordinate CA certificates that are allowed. If this value is less than 0, the request will fail. If this value is missing, the max path length will be omitted from the CA certificate.
    #[builder(into, default)]
    #[serde(rename = "maxIssuerPathLength")]
    pub r#max_issuer_path_length: Box<Option<i32>>,
}
