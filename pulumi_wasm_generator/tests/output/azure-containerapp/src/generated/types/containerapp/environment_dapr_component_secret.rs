#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct EnvironmentDaprComponentSecret {
    /// The identity to use for accessing key vault reference.
    #[builder(into, default)]
    #[serde(rename = "identity")]
    pub r#identity: Box<Option<String>>,
    /// The Key Vault Secret ID. Could be either one of `id` or `versionless_id`.
    #[builder(into, default)]
    #[serde(rename = "keyVaultSecretId")]
    pub r#key_vault_secret_id: Box<Option<String>>,
    /// The Secret name.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// The value for this secret.
    #[builder(into, default)]
    #[serde(rename = "value")]
    pub r#value: Box<Option<String>>,
}
