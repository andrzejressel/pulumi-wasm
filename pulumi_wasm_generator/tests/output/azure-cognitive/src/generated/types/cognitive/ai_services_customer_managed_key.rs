#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct AiServicesCustomerManagedKey {
    /// The Client ID of the User Assigned Identity that has access to the key. This property only needs to be specified when there are multiple identities attached to the Azure AI Service.
    #[builder(into, default)]
    #[serde(rename = "identityClientId")]
    pub r#identity_client_id: Box<Option<String>>,
    /// The ID of the Key Vault Key which should be used to encrypt the data in this AI Services Account. Exactly one of `key_vault_key_id`, `managed_hsm_key_id` must be specified.
    #[builder(into, default)]
    #[serde(rename = "keyVaultKeyId")]
    pub r#key_vault_key_id: Box<Option<String>>,
    /// The ID of the managed HSM Key which should be used to encrypt the data in this AI Services Account. Exactly one of `key_vault_key_id`, `managed_hsm_key_id` must be specified.
    #[builder(into, default)]
    #[serde(rename = "managedHsmKeyId")]
    pub r#managed_hsm_key_id: Box<Option<String>>,
}
