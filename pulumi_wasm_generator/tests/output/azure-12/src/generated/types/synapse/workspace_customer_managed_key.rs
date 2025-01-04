#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct WorkspaceCustomerManagedKey {
    /// An identifier for the key. Name needs to match the name of the key used with the `azure.synapse.WorkspaceKey` resource. Defaults to "cmk" if not specified.
    #[builder(into, default)]
    #[serde(rename = "keyName")]
    pub r#key_name: Box<Option<String>>,
    /// The Azure Key Vault Key Versionless ID to be used as the Customer Managed Key (CMK) for double encryption (e.g. `https://example-keyvault.vault.azure.net/type/cmk/`).
    #[builder(into)]
    #[serde(rename = "keyVersionlessId")]
    pub r#key_versionless_id: Box<String>,
    /// The User Assigned Identity ID to be used for accessing the Customer Managed Key for encryption.
    #[builder(into, default)]
    #[serde(rename = "userAssignedIdentityId")]
    pub r#user_assigned_identity_id: Box<Option<String>>,
}
