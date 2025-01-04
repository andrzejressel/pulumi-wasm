#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct NetworkManagerConnectivityConfigurationAppliesToGroup {
    /// Indicates whether to global mesh is supported for this group. Possible values are `true` and `false`.
    /// 
    /// > **NOTE:** A group can be global only if the `group_connectivity` is `DirectlyConnected`.
    #[builder(into, default)]
    #[serde(rename = "globalMeshEnabled")]
    pub r#global_mesh_enabled: Box<Option<bool>>,
    /// Specifies the group connectivity type. Possible values are `None` and `DirectlyConnected`.
    #[builder(into)]
    #[serde(rename = "groupConnectivity")]
    pub r#group_connectivity: Box<String>,
    /// Specifies the resource ID of Network Group which the configuration applies to.
    #[builder(into)]
    #[serde(rename = "networkGroupId")]
    pub r#network_group_id: Box<String>,
    /// Indicates whether the hub gateway is used. Possible values are `true` and `false`.
    #[builder(into, default)]
    #[serde(rename = "useHubGateway")]
    pub r#use_hub_gateway: Box<Option<bool>>,
}
