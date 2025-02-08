#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct AccountManagedResource {
    /// The ID of the managed event hub namespace.
    #[builder(into, default)]
    #[serde(rename = "eventHubNamespaceId")]
    pub r#event_hub_namespace_id: Box<Option<String>>,
    /// The ID of the managed resource group.
    #[builder(into, default)]
    #[serde(rename = "resourceGroupId")]
    pub r#resource_group_id: Box<Option<String>>,
    /// The ID of the managed storage account.
    #[builder(into, default)]
    #[serde(rename = "storageAccountId")]
    pub r#storage_account_id: Box<Option<String>>,
}
