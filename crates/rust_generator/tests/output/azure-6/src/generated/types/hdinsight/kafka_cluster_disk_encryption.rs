#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct KafkaClusterDiskEncryption {
    /// This is an algorithm identifier for encryption. Possible values are `RSA1_5`, `RSA-OAEP`, `RSA-OAEP-256`.
    #[builder(into, default)]
    #[serde(rename = "encryptionAlgorithm")]
    pub r#encryption_algorithm: Box<Option<String>>,
    /// This is indicator to show whether resource disk encryption is enabled.
    #[builder(into, default)]
    #[serde(rename = "encryptionAtHostEnabled")]
    pub r#encryption_at_host_enabled: Box<Option<bool>>,
    /// The ID of the key vault key.
    #[builder(into, default)]
    #[serde(rename = "keyVaultKeyId")]
    pub r#key_vault_key_id: Box<Option<String>>,
    /// This is the resource ID of Managed Identity used to access the key vault.
    #[builder(into, default)]
    #[serde(rename = "keyVaultManagedIdentityId")]
    pub r#key_vault_managed_identity_id: Box<Option<String>>,
}
