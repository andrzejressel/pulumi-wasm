#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct DefinitionPermission {
    /// One or more Allowed Actions, such as `*`, `Microsoft.Resources/subscriptions/resourceGroups/read`. See ['Azure Resource Manager resource provider operations'](https://docs.microsoft.com/azure/role-based-access-control/resource-provider-operations) for details.
    #[builder(into, default)]
    #[serde(rename = "actions")]
    pub r#actions: Box<Option<Vec<String>>>,
    /// One or more Allowed Data Actions, such as `*`, `Microsoft.Storage/storageAccounts/blobServices/containers/blobs/read`. See ['Azure Resource Manager resource provider operations'](https://docs.microsoft.com/azure/role-based-access-control/resource-provider-operations) for details.
    #[builder(into, default)]
    #[serde(rename = "dataActions")]
    pub r#data_actions: Box<Option<Vec<String>>>,
    /// One or more Disallowed Actions, such as `*`, `Microsoft.Resources/subscriptions/resourceGroups/read`. See ['Azure Resource Manager resource provider operations'](https://docs.microsoft.com/azure/role-based-access-control/resource-provider-operations) for details.
    #[builder(into, default)]
    #[serde(rename = "notActions")]
    pub r#not_actions: Box<Option<Vec<String>>>,
    /// One or more Disallowed Data Actions, such as `*`, `Microsoft.Resources/subscriptions/resourceGroups/read`. See ['Azure Resource Manager resource provider operations'](https://docs.microsoft.com/azure/role-based-access-control/resource-provider-operations) for details.
    #[builder(into, default)]
    #[serde(rename = "notDataActions")]
    pub r#not_data_actions: Box<Option<Vec<String>>>,
}
