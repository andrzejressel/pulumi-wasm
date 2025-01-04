#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct SystemTopicEventSubscriptionStorageBlobDeadLetterDestination {
    /// Specifies the id of the storage account id where the storage blob is located.
    #[builder(into)]
    #[serde(rename = "storageAccountId")]
    pub r#storage_account_id: Box<String>,
    /// Specifies the name of the Storage blob container that is the destination of the deadletter events.
    #[builder(into)]
    #[serde(rename = "storageBlobContainerName")]
    pub r#storage_blob_container_name: Box<String>,
}
