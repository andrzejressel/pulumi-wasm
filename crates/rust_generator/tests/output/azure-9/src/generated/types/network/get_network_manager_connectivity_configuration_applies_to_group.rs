#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct GetNetworkManagerConnectivityConfigurationAppliesToGroup {
    /// Whether global mesh is supported.
    #[builder(into)]
    #[serde(rename = "globalMeshEnabled")]
    pub r#global_mesh_enabled: Box<bool>,
    /// The group connectivity type.
    #[builder(into)]
    #[serde(rename = "groupConnectivity")]
    pub r#group_connectivity: Box<String>,
    /// The ID of the Network Manager Network Group.
    #[builder(into)]
    #[serde(rename = "networkGroupId")]
    pub r#network_group_id: Box<String>,
    /// Whether hub gateway is used.
    #[builder(into)]
    #[serde(rename = "useHubGateway")]
    pub r#use_hub_gateway: Box<bool>,
}
