#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct GetVolumeGroupEncryption {
    /// The timestamp of the expiration time for the current version of the Customer Managed Key.
    #[builder(into)]
    #[serde(rename = "currentVersionedKeyExpirationTimestamp")]
    pub r#current_versioned_key_expiration_timestamp: Box<String>,
    /// The ID of the current versioned Key Vault Key in use.
    #[builder(into)]
    #[serde(rename = "currentVersionedKeyId")]
    pub r#current_versioned_key_id: Box<String>,
    /// The Key Vault Key URI for Customer Managed Key encryption, which can be either a full URI or a versionless URI.
    #[builder(into)]
    #[serde(rename = "keyVaultKeyId")]
    pub r#key_vault_key_id: Box<String>,
    /// The timestamp of the last rotation of the Key Vault Key.
    #[builder(into)]
    #[serde(rename = "lastKeyRotationTimestamp")]
    pub r#last_key_rotation_timestamp: Box<String>,
    /// The ID of the User Assigned Identity used by this Elastic SAN Volume Group.
    #[builder(into)]
    #[serde(rename = "userAssignedIdentityId")]
    pub r#user_assigned_identity_id: Box<String>,
}
