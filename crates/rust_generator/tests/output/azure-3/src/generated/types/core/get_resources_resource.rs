#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetResourcesResource {
    /// The ID of this Resource.
    #[builder(into)]
    #[serde(rename = "id")]
    pub r#id: Box<String>,
    /// The Azure Region in which this Resource exists.
    #[builder(into)]
    #[serde(rename = "location")]
    pub r#location: Box<String>,
    /// The name of the Resource.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// The name of the Resource group where the Resources are located.
    #[builder(into)]
    #[serde(rename = "resourceGroupName")]
    pub r#resource_group_name: Box<String>,
    /// A map of tags assigned to this Resource.
    #[builder(into)]
    #[serde(rename = "tags")]
    pub r#tags: Box<std::collections::HashMap<String, String>>,
    /// The Resource Type of the Resources you want to list (e.g. `Microsoft.Network/virtualNetworks`). A resource type's name follows the format: `{resource-provider}/{resource-type}`. The resource type for a key vault is `Microsoft.KeyVault/vaults`. A full list of available Resource Providers can be found [here](https://docs.microsoft.com/azure/azure-resource-manager/azure-services-resource-providers). A full list of Resources Types can be found [here](https://learn.microsoft.com/en-us/azure/templates/#find-resources).
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: Box<String>,
}
