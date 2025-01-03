#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetClusterNodePoolNodeConfigContainerdConfigPrivateRegistryAccessConfig {
    /// Parameters for configuring CA certificate and domains.
    #[builder(into)]
    #[serde(rename = "certificateAuthorityDomainConfigs")]
    pub r#certificate_authority_domain_configs: Box<Vec<super::super::types::container::GetClusterNodePoolNodeConfigContainerdConfigPrivateRegistryAccessConfigCertificateAuthorityDomainConfig>>,
    /// Whether or not private registries are configured.
    #[builder(into)]
    #[serde(rename = "enabled")]
    pub r#enabled: Box<bool>,
}
