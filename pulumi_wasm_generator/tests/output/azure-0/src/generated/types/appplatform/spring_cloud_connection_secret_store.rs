#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct SpringCloudConnectionSecretStore {
    /// The key vault id to store secret.
    #[builder(into)]
    #[serde(rename = "keyVaultId")]
    pub r#key_vault_id: Box<String>,
}
