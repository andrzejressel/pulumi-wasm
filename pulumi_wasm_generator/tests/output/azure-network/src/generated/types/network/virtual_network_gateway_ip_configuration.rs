#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct VirtualNetworkGatewayIpConfiguration {
    /// A user-defined name of the IP configuration. Defaults to `vnetGatewayConfig`.
    #[builder(into, default)]
    #[serde(rename = "name")]
    pub r#name: Box<Option<String>>,
    /// Defines how the private IP address of the gateways virtual interface is assigned. The only valid value is `Dynamic` for Virtual Network Gateway (`Static` is not supported by the service yet). Defaults to `Dynamic`.
    #[builder(into, default)]
    #[serde(rename = "privateIpAddressAllocation")]
    pub r#private_ip_address_allocation: Box<Option<String>>,
    /// The ID of the public IP address to associate with the Virtual Network Gateway.
    #[builder(into)]
    #[serde(rename = "publicIpAddressId")]
    pub r#public_ip_address_id: Box<String>,
    /// The ID of the gateway subnet of a virtual network in which the virtual network gateway will be created. It is mandatory that the associated subnet is named `GatewaySubnet`. Therefore, each virtual network can contain at most a single Virtual Network Gateway.
    #[builder(into)]
    #[serde(rename = "subnetId")]
    pub r#subnet_id: Box<String>,
}
