#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct AiServicesStorage {
    /// The client ID of the Managed Identity associated with the Storage Account.
    #[builder(into, default)]
    #[serde(rename = "identityClientId")]
    pub r#identity_client_id: Box<Option<String>>,
    /// The ID of the Storage Account.
    #[builder(into)]
    #[serde(rename = "storageAccountId")]
    pub r#storage_account_id: Box<String>,
}
