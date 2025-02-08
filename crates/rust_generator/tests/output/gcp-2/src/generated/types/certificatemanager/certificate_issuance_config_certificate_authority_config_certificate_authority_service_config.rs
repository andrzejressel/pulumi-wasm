#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct CertificateIssuanceConfigCertificateAuthorityConfigCertificateAuthorityServiceConfig {
    /// A CA pool resource used to issue a certificate.
    /// The CA pool string has a relative resource path following the form
    /// "projects/{project}/locations/{location}/caPools/{caPool}".
    /// 
    /// - - -
    #[builder(into)]
    #[serde(rename = "caPool")]
    pub r#ca_pool: Box<String>,
}
