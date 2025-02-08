#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct NamespaceCustomerManagedKey {
    /// The ID of the User Assigned Identity that has access to the key.
    #[builder(into)]
    #[serde(rename = "identityId")]
    pub r#identity_id: Box<String>,
    /// Used to specify whether enable Infrastructure Encryption (Double Encryption). Changing this forces a new resource to be created.
    #[builder(into, default)]
    #[serde(rename = "infrastructureEncryptionEnabled")]
    pub r#infrastructure_encryption_enabled: Box<Option<bool>>,
    /// The ID of the Key Vault Key which should be used to Encrypt the data in this ServiceBus Namespace.
    #[builder(into)]
    #[serde(rename = "keyVaultKeyId")]
    pub r#key_vault_key_id: Box<String>,
}
