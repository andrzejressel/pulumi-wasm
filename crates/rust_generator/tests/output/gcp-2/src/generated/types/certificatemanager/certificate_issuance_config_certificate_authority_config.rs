#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct CertificateIssuanceConfigCertificateAuthorityConfig {
    /// Defines a CertificateAuthorityServiceConfig.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "certificateAuthorityServiceConfig")]
    pub r#certificate_authority_service_config: Box<Option<super::super::types::certificatemanager::CertificateIssuanceConfigCertificateAuthorityConfigCertificateAuthorityServiceConfig>>,
}
