#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct KubernetesClusterServiceMeshProfileCertificateAuthority {
    /// The certificate chain object name in Azure Key Vault.
    #[builder(into)]
    #[serde(rename = "certChainObjectName")]
    pub r#cert_chain_object_name: Box<String>,
    /// The intermediate certificate object name in Azure Key Vault.
    #[builder(into)]
    #[serde(rename = "certObjectName")]
    pub r#cert_object_name: Box<String>,
    /// The intermediate certificate private key object name in Azure Key Vault.
    /// 
    /// > **Note:** For more information on [Istio-based service mesh add-on with plug-in CA certificates and how to generate these certificates](https://learn.microsoft.com/en-us/azure/aks/istio-plugin-ca),
    #[builder(into)]
    #[serde(rename = "keyObjectName")]
    pub r#key_object_name: Box<String>,
    /// The resource ID of the Key Vault.
    #[builder(into)]
    #[serde(rename = "keyVaultId")]
    pub r#key_vault_id: Box<String>,
    /// The root certificate object name in Azure Key Vault.
    #[builder(into)]
    #[serde(rename = "rootCertObjectName")]
    pub r#root_cert_object_name: Box<String>,
}
