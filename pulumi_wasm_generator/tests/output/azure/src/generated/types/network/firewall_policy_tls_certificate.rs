#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct FirewallPolicyTlsCertificate {
    /// The ID of the Key Vault, where the secret or certificate is stored.
    #[builder(into)]
    #[serde(rename = "keyVaultSecretId")]
    pub r#key_vault_secret_id: Box<String>,
    /// The name of the certificate.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
}