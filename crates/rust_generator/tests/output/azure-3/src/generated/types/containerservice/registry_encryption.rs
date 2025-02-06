#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct RegistryEncryption {
    /// The client ID of the managed identity associated with the encryption key.
    #[builder(into)]
    #[serde(rename = "identityClientId")]
    pub r#identity_client_id: Box<String>,
    /// The ID of the Key Vault Key.
    #[builder(into)]
    #[serde(rename = "keyVaultKeyId")]
    pub r#key_vault_key_id: Box<String>,
}
