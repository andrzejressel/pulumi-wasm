#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct ApplicationGatewaySslCertificate {
    /// The base64-encoded PFX certificate data. Required if `key_vault_secret_id` is not set.
    /// 
    /// > **NOTE:** When specifying a file, use `data = filebase64("path/to/file")` to encode the contents of that file.
    #[builder(into, default)]
    #[serde(rename = "data")]
    pub r#data: Box<Option<String>>,
    /// The ID of the Rewrite Rule Set
    #[builder(into, default)]
    #[serde(rename = "id")]
    pub r#id: Box<Option<String>>,
    /// The Secret ID of the (base-64 encoded unencrypted pfx) `Secret` or `Certificate` object stored in Azure KeyVault. You need to enable soft delete for Key Vault to use this feature. Required if `data` is not set.
    /// 
    /// > **NOTE:** To implement certificate rotation, `versionless_secret_id` should be used, although `secret_id` is also supported.
    /// 
    /// > **NOTE:** TLS termination with Key Vault certificates is limited to the [v2 SKUs](https://docs.microsoft.com/azure/application-gateway/key-vault-certs).
    /// 
    /// > **NOTE:** For TLS termination with Key Vault certificates to work properly, an existing user-assigned managed identity, which Application Gateway uses to retrieve certificates from Key Vault, should be defined via `identity` block. Additionally, access policies in the Key Vault to allow the identity to be granted *get* access to the secret should be defined.
    #[builder(into, default)]
    #[serde(rename = "keyVaultSecretId")]
    pub r#key_vault_secret_id: Box<Option<String>>,
    /// The Name of the SSL certificate that is unique within this Application Gateway
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// Password for the pfx file specified in data. Required if `data` is set.
    #[builder(into, default)]
    #[serde(rename = "password")]
    pub r#password: Box<Option<String>>,
    /// The Public Certificate Data associated with the SSL Certificate.
    #[builder(into, default)]
    #[serde(rename = "publicCertData")]
    pub r#public_cert_data: Box<Option<String>>,
}
