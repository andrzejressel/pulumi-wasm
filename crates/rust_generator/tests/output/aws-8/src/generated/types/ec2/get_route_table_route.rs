#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct GetRouteTableRoute {
    /// ID of the Carrier Gateway.
    #[builder(into)]
    #[serde(rename = "carrierGatewayId")]
    pub r#carrier_gateway_id: Box<String>,
    /// CIDR block of the route.
    #[builder(into)]
    #[serde(rename = "cidrBlock")]
    pub r#cidr_block: Box<String>,
    /// ARN of the core network.
    #[builder(into)]
    #[serde(rename = "coreNetworkArn")]
    pub r#core_network_arn: Box<String>,
    /// The ID of a managed prefix list destination of the route.
    #[builder(into)]
    #[serde(rename = "destinationPrefixListId")]
    pub r#destination_prefix_list_id: Box<String>,
    /// ID of the Egress Only Internet Gateway.
    #[builder(into)]
    #[serde(rename = "egressOnlyGatewayId")]
    pub r#egress_only_gateway_id: Box<String>,
    /// ID of an Internet Gateway or Virtual Private Gateway which is connected to the Route Table (not exported if not passed as a parameter).
    #[builder(into)]
    #[serde(rename = "gatewayId")]
    pub r#gateway_id: Box<String>,
    /// EC2 instance ID.
    #[builder(into)]
    #[serde(rename = "instanceId")]
    pub r#instance_id: Box<String>,
    /// IPv6 CIDR block of the route.
    #[builder(into)]
    #[serde(rename = "ipv6CidrBlock")]
    pub r#ipv_6_cidr_block: Box<String>,
    /// Local Gateway ID.
    #[builder(into)]
    #[serde(rename = "localGatewayId")]
    pub r#local_gateway_id: Box<String>,
    /// NAT Gateway ID.
    #[builder(into)]
    #[serde(rename = "natGatewayId")]
    pub r#nat_gateway_id: Box<String>,
    /// ID of the elastic network interface (eni) to use.
    #[builder(into)]
    #[serde(rename = "networkInterfaceId")]
    pub r#network_interface_id: Box<String>,
    /// EC2 Transit Gateway ID.
    #[builder(into)]
    #[serde(rename = "transitGatewayId")]
    pub r#transit_gateway_id: Box<String>,
    /// VPC Endpoint ID.
    #[builder(into)]
    #[serde(rename = "vpcEndpointId")]
    pub r#vpc_endpoint_id: Box<String>,
    /// VPC Peering ID.
    #[builder(into)]
    #[serde(rename = "vpcPeeringConnectionId")]
    pub r#vpc_peering_connection_id: Box<String>,
}
