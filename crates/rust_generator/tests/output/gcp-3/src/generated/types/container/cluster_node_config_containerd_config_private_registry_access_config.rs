#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ClusterNodeConfigContainerdConfigPrivateRegistryAccessConfig {
    /// List of configuration objects for CA and domains. Each object identifies a certificate and its assigned domains. See [how to configure for private container registries](https://cloud.google.com/kubernetes-engine/docs/how-to/access-private-registries-private-certificates) for more detail. Example: 
    #[builder(into, default)]
    #[serde(rename = "certificateAuthorityDomainConfigs")]
    pub r#certificate_authority_domain_configs: Box<Option<Vec<super::super::types::container::ClusterNodeConfigContainerdConfigPrivateRegistryAccessConfigCertificateAuthorityDomainConfig>>>,
    /// Enables private registry config. If set to false, all other fields in this object must not be set.
    #[builder(into)]
    #[serde(rename = "enabled")]
    pub r#enabled: Box<bool>,
}
