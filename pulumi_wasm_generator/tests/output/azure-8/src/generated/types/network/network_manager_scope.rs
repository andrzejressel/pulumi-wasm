#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct NetworkManagerScope {
    /// A list of management group IDs.
    /// 
    /// > **NOTE:** When specifying a scope at the management group level, you need to register the `Microsoft.Network` at the management group scope before deploying a Network Manager, more information can be found in the [Azure document](https://learn.microsoft.com/en-us/azure/virtual-network-manager/concept-network-manager-scope#scope).
    #[builder(into, default)]
    #[serde(rename = "managementGroupIds")]
    pub r#management_group_ids: Box<Option<Vec<String>>>,
    /// A list of subscription IDs.
    #[builder(into, default)]
    #[serde(rename = "subscriptionIds")]
    pub r#subscription_ids: Box<Option<Vec<String>>>,
}
