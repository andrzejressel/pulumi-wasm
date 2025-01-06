#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct RouteTableRoute {
    /// The destination to which the route applies. Can be CIDR (such as `10.1.0.0/16`) or [Azure Service Tag](https://docs.microsoft.com/azure/virtual-network/service-tags-overview) (such as `ApiManagement`, `AzureBackup` or `AzureMonitor`) format.
    #[builder(into)]
    #[serde(rename = "addressPrefix")]
    pub r#address_prefix: Box<String>,
    /// The name of the route.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// Contains the IP address packets should be forwarded to. Next hop values are only allowed in routes where the next hop type is `VirtualAppliance`.
    #[builder(into, default)]
    #[serde(rename = "nextHopInIpAddress")]
    pub r#next_hop_in_ip_address: Box<Option<String>>,
    /// The type of Azure hop the packet should be sent to. Possible values are `VirtualNetworkGateway`, `VnetLocal`, `Internet`, `VirtualAppliance` and `None`.
    #[builder(into)]
    #[serde(rename = "nextHopType")]
    pub r#next_hop_type: Box<String>,
}
