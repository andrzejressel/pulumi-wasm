#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetAccountEncryption {
    /// The full URL path of the Key Vault Key used to encrypt data for this Batch account.
    #[builder(into)]
    #[serde(rename = "keyVaultKeyId")]
    pub r#key_vault_key_id: Box<String>,
}
