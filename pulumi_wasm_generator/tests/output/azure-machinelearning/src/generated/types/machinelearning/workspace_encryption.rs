#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct WorkspaceEncryption {
    /// The Key Vault URI to access the encryption key.
    #[builder(into)]
    #[serde(rename = "keyId")]
    pub r#key_id: Box<String>,
    /// The ID of the keyVault where the customer owned encryption key is present.
    #[builder(into)]
    #[serde(rename = "keyVaultId")]
    pub r#key_vault_id: Box<String>,
    /// The Key Vault URI to access the encryption key.
    /// 
    /// > **Note:** `user_assigned_identity_id` must set when`identity.type` is `UserAssigned` or service won't be able to find the assigned permissions.
    #[builder(into, default)]
    #[serde(rename = "userAssignedIdentityId")]
    pub r#user_assigned_identity_id: Box<Option<String>>,
}
