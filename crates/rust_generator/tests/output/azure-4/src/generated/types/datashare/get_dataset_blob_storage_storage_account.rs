#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetDatasetBlobStorageStorageAccount {
    /// The name of this Data Share Blob Storage Dataset.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// The resource group name of the storage account to be shared with the receiver.
    #[builder(into)]
    #[serde(rename = "resourceGroupName")]
    pub r#resource_group_name: Box<String>,
    /// The subscription id of the storage account to be shared with the receiver.
    #[builder(into)]
    #[serde(rename = "subscriptionId")]
    pub r#subscription_id: Box<String>,
}
