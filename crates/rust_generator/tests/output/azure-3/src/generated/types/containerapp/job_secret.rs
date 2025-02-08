#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct JobSecret {
    /// The identity to use for accessing the Key Vault secret reference. This can either be the Resource ID of a User Assigned Identity, or `System` for the System Assigned Identity.
    /// 
    /// !> **Note:** `identity` must be used together with `key_vault_secret_id`
    #[builder(into, default)]
    #[serde(rename = "identity")]
    pub r#identity: Box<Option<String>>,
    /// The ID of a Key Vault secret. This can be a versioned or version-less ID.
    /// 
    /// !> **Note:** When using `key_vault_secret_id`, `ignore_changes` should be used to ignore any changes to `value`.
    #[builder(into, default)]
    #[serde(rename = "keyVaultSecretId")]
    pub r#key_vault_secret_id: Box<Option<String>>,
    /// The secret name.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// The value for this secret.
    /// 
    /// !> **Note:** `value` will be ignored if `key_vault_secret_id` and `identity` are provided.
    #[builder(into, default)]
    #[serde(rename = "value")]
    pub r#value: Box<Option<String>>,
}
