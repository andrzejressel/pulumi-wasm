#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct DefaultRouteTableRoute {
    /// The CIDR block of the route.
    #[builder(into, default)]
    #[serde(rename = "cidrBlock")]
    pub r#cidr_block: Box<Option<String>>,
    /// The Amazon Resource Name (ARN) of a core network.
    #[builder(into, default)]
    #[serde(rename = "coreNetworkArn")]
    pub r#core_network_arn: Box<Option<String>>,
    /// The ID of a managed prefix list destination of the route.
    /// 
    /// One of the following target arguments must be supplied:
    #[builder(into, default)]
    #[serde(rename = "destinationPrefixListId")]
    pub r#destination_prefix_list_id: Box<Option<String>>,
    /// Identifier of a VPC Egress Only Internet Gateway.
    #[builder(into, default)]
    #[serde(rename = "egressOnlyGatewayId")]
    pub r#egress_only_gateway_id: Box<Option<String>>,
    /// Identifier of a VPC internet gateway or a virtual private gateway.
    #[builder(into, default)]
    #[serde(rename = "gatewayId")]
    pub r#gateway_id: Box<Option<String>>,
    /// Identifier of an EC2 instance.
    #[builder(into, default)]
    #[serde(rename = "instanceId")]
    pub r#instance_id: Box<Option<String>>,
    /// The Ipv6 CIDR block of the route
    #[builder(into, default)]
    #[serde(rename = "ipv6CidrBlock")]
    pub r#ipv_6_cidr_block: Box<Option<String>>,
    /// Identifier of a VPC NAT gateway.
    #[builder(into, default)]
    #[serde(rename = "natGatewayId")]
    pub r#nat_gateway_id: Box<Option<String>>,
    /// Identifier of an EC2 network interface.
    #[builder(into, default)]
    #[serde(rename = "networkInterfaceId")]
    pub r#network_interface_id: Box<Option<String>>,
    /// Identifier of an EC2 Transit Gateway.
    #[builder(into, default)]
    #[serde(rename = "transitGatewayId")]
    pub r#transit_gateway_id: Box<Option<String>>,
    /// Identifier of a VPC Endpoint. This route must be removed prior to VPC Endpoint deletion.
    #[builder(into, default)]
    #[serde(rename = "vpcEndpointId")]
    pub r#vpc_endpoint_id: Box<Option<String>>,
    /// Identifier of a VPC peering connection.
    /// 
    /// Note that the default route, mapping the VPC's CIDR block to "local", is created implicitly and cannot be specified.
    #[builder(into, default)]
    #[serde(rename = "vpcPeeringConnectionId")]
    pub r#vpc_peering_connection_id: Box<Option<String>>,
}
