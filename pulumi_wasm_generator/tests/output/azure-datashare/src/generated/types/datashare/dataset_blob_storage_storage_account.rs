#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct DatasetBlobStorageStorageAccount {
    /// The name of the storage account to be shared with the receiver. Changing this forces a new Data Share Blob Storage Dataset to be created.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// The resource group name of the storage account to be shared with the receiver. Changing this forces a new Data Share Blob Storage Dataset to be created.
    #[builder(into)]
    #[serde(rename = "resourceGroupName")]
    pub r#resource_group_name: Box<String>,
    /// The subscription id of the storage account to be shared with the receiver. Changing this forces a new Data Share Blob Storage Dataset to be created.
    #[builder(into)]
    #[serde(rename = "subscriptionId")]
    pub r#subscription_id: Box<String>,
}