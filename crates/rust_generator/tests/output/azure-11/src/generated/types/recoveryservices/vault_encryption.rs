#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct VaultEncryption {
    /// Enabling/Disabling the Double Encryption state.
    #[builder(into)]
    #[serde(rename = "infrastructureEncryptionEnabled")]
    pub r#infrastructure_encryption_enabled: Box<bool>,
    /// The Key Vault key id used to encrypt this vault. Key managed by Vault Managed Hardware Security Module is also supported.
    #[builder(into)]
    #[serde(rename = "keyId")]
    pub r#key_id: Box<String>,
    /// Indicate that system assigned identity should be used or not. Defaults to `true`. Must be set to `false` when `user_assigned_identity_id` is set.
    /// 
    /// !> **Note:** `use_system_assigned_identity` only be able to set to `false` for **new** vaults. Any vaults containing existing items registered or attempted to be registered to it are not supported. Details can be found in [the document](https://learn.microsoft.com/en-us/azure/backup/encryption-at-rest-with-cmk?tabs=portal#before-you-start)
    /// 
    /// !> **Note:** Once `infrastructure_encryption_enabled` has been set it's not possible to change it.
    #[builder(into, default)]
    #[serde(rename = "useSystemAssignedIdentity")]
    pub r#use_system_assigned_identity: Box<Option<bool>>,
    /// Specifies the user assigned identity ID to be used.
    #[builder(into, default)]
    #[serde(rename = "userAssignedIdentityId")]
    pub r#user_assigned_identity_id: Box<Option<String>>,
}
