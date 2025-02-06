#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetVirtualNetworkGatewayIpConfiguration {
    /// The resource ID of the IP configuration.
    #[builder(into)]
    #[serde(rename = "id")]
    pub r#id: Box<String>,
    /// Specifies the name of the Virtual Network Gateway.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// The Private IP Address associated with the Virtual Network Gateway.
    #[builder(into)]
    #[serde(rename = "privateIpAddress")]
    pub r#private_ip_address: Box<String>,
    /// Defines how the private IP address
    /// of the gateways virtual interface is assigned.
    #[builder(into)]
    #[serde(rename = "privateIpAddressAllocation")]
    pub r#private_ip_address_allocation: Box<String>,
    /// The ID of the Public IP Address associated
    /// with the Virtual Network Gateway.
    #[builder(into)]
    #[serde(rename = "publicIpAddressId")]
    pub r#public_ip_address_id: Box<String>,
    /// The ID of the gateway subnet of a virtual network in
    /// which the virtual network gateway will be created. It is mandatory that
    /// the associated subnet is named `GatewaySubnet`. Therefore, each virtual
    /// network can contain at most a single Virtual Network Gateway.
    #[builder(into)]
    #[serde(rename = "subnetId")]
    pub r#subnet_id: Box<String>,
}
