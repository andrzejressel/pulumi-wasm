#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ClusterNodeConfigContainerdConfigPrivateRegistryAccessConfigCertificateAuthorityDomainConfig {
    /// List of fully-qualified-domain-names. IPv4s and port specification are supported.
    #[builder(into)]
    #[serde(rename = "fqdns")]
    pub r#fqdns: Box<Vec<String>>,
    /// Parameters for configuring a certificate hosted in GCP SecretManager.
    #[builder(into)]
    #[serde(rename = "gcpSecretManagerCertificateConfig")]
    pub r#gcp_secret_manager_certificate_config: Box<super::super::types::container::ClusterNodeConfigContainerdConfigPrivateRegistryAccessConfigCertificateAuthorityDomainConfigGcpSecretManagerCertificateConfig>,
}
