#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetConfigurationStoreEncryption {
    #[builder(into)]
    #[serde(rename = "identityClientId")]
    pub r#identity_client_id: Box<String>,
    #[builder(into)]
    #[serde(rename = "keyVaultKeyIdentifier")]
    pub r#key_vault_key_identifier: Box<String>,
}
