#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct AccountCustomerManagedKey {
    /// The ID of the Key Vault Key, supplying a version-less key ID will enable auto-rotation of this key. Exactly one of `key_vault_key_id` and `managed_hsm_key_id` may be specified.
    #[builder(into, default)]
    #[serde(rename = "keyVaultKeyId")]
    pub r#key_vault_key_id: Box<Option<String>>,
    /// The ID of the managed HSM Key. Exactly one of `key_vault_key_id` and `managed_hsm_key_id` may be specified.
    #[builder(into, default)]
    #[serde(rename = "managedHsmKeyId")]
    pub r#managed_hsm_key_id: Box<Option<String>>,
    /// The ID of a user assigned identity.
    /// 
    /// > **Note:** `customer_managed_key` can only be set when the `account_kind` is set to `StorageV2` or `account_tier` set to `Premium`, and the identity type is `UserAssigned`.
    #[builder(into)]
    #[serde(rename = "userAssignedIdentityId")]
    pub r#user_assigned_identity_id: Box<String>,
}
