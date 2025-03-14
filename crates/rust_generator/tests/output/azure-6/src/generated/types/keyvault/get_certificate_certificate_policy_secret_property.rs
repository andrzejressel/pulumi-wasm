#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetCertificateCertificatePolicySecretProperty {
    /// The Content-Type of the Certificate, for example `application/x-pkcs12` for a PFX or `application/x-pem-file` for a PEM.
    #[builder(into)]
    #[serde(rename = "contentType")]
    pub r#content_type: Box<String>,
}
