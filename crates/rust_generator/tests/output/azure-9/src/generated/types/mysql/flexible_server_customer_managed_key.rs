#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct FlexibleServerCustomerManagedKey {
    /// The ID of the geo backup Key Vault Key. It can't cross region and need Customer Managed Key in same region as geo backup.
    #[builder(into, default)]
    #[serde(rename = "geoBackupKeyVaultKeyId")]
    pub r#geo_backup_key_vault_key_id: Box<Option<String>>,
    /// The geo backup user managed identity id for a Customer Managed Key. Should be added with `identity_ids`. It can't cross region and need identity in same region as geo backup.
    /// 
    /// > **NOTE:** `primary_user_assigned_identity_id` or `geo_backup_user_assigned_identity_id` is required when `type` is set to `UserAssigned` or `SystemAssigned, UserAssigned`.
    #[builder(into, default)]
    #[serde(rename = "geoBackupUserAssignedIdentityId")]
    pub r#geo_backup_user_assigned_identity_id: Box<Option<String>>,
    /// The ID of the Key Vault Key.
    #[builder(into, default)]
    #[serde(rename = "keyVaultKeyId")]
    pub r#key_vault_key_id: Box<Option<String>>,
    /// Specifies the primary user managed identity id for a Customer Managed Key. Should be added with `identity_ids`.
    #[builder(into, default)]
    #[serde(rename = "primaryUserAssignedIdentityId")]
    pub r#primary_user_assigned_identity_id: Box<Option<String>>,
}
