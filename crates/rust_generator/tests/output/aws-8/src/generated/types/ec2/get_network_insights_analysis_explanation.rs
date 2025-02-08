#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetNetworkInsightsAnalysisExplanation {
    #[builder(into)]
    #[serde(rename = "aclRules")]
    pub r#acl_rules: Box<Vec<super::super::types::ec2::GetNetworkInsightsAnalysisExplanationAclRule>>,
    #[builder(into)]
    #[serde(rename = "acls")]
    pub r#acls: Box<Vec<super::super::types::ec2::GetNetworkInsightsAnalysisExplanationAcl>>,
    #[builder(into)]
    #[serde(rename = "address")]
    pub r#address: Box<String>,
    #[builder(into)]
    #[serde(rename = "addresses")]
    pub r#addresses: Box<Vec<String>>,
    #[builder(into)]
    #[serde(rename = "attachedTos")]
    pub r#attached_tos: Box<Vec<super::super::types::ec2::GetNetworkInsightsAnalysisExplanationAttachedTo>>,
    #[builder(into)]
    #[serde(rename = "availabilityZones")]
    pub r#availability_zones: Box<Vec<String>>,
    #[builder(into)]
    #[serde(rename = "cidrs")]
    pub r#cidrs: Box<Vec<String>>,
    #[builder(into)]
    #[serde(rename = "classicLoadBalancerListeners")]
    pub r#classic_load_balancer_listeners: Box<Vec<super::super::types::ec2::GetNetworkInsightsAnalysisExplanationClassicLoadBalancerListener>>,
    #[builder(into)]
    #[serde(rename = "components")]
    pub r#components: Box<Vec<super::super::types::ec2::GetNetworkInsightsAnalysisExplanationComponent>>,
    #[builder(into)]
    #[serde(rename = "customerGateways")]
    pub r#customer_gateways: Box<Vec<super::super::types::ec2::GetNetworkInsightsAnalysisExplanationCustomerGateway>>,
    #[builder(into)]
    #[serde(rename = "destinationVpcs")]
    pub r#destination_vpcs: Box<Vec<super::super::types::ec2::GetNetworkInsightsAnalysisExplanationDestinationVpc>>,
    #[builder(into)]
    #[serde(rename = "destinations")]
    pub r#destinations: Box<Vec<super::super::types::ec2::GetNetworkInsightsAnalysisExplanationDestination>>,
    #[builder(into)]
    #[serde(rename = "direction")]
    pub r#direction: Box<String>,
    #[builder(into)]
    #[serde(rename = "elasticLoadBalancerListeners")]
    pub r#elastic_load_balancer_listeners: Box<Vec<super::super::types::ec2::GetNetworkInsightsAnalysisExplanationElasticLoadBalancerListener>>,
    #[builder(into)]
    #[serde(rename = "explanationCode")]
    pub r#explanation_code: Box<String>,
    #[builder(into)]
    #[serde(rename = "ingressRouteTables")]
    pub r#ingress_route_tables: Box<Vec<super::super::types::ec2::GetNetworkInsightsAnalysisExplanationIngressRouteTable>>,
    #[builder(into)]
    #[serde(rename = "internetGateways")]
    pub r#internet_gateways: Box<Vec<super::super::types::ec2::GetNetworkInsightsAnalysisExplanationInternetGateway>>,
    #[builder(into)]
    #[serde(rename = "loadBalancerArn")]
    pub r#load_balancer_arn: Box<String>,
    #[builder(into)]
    #[serde(rename = "loadBalancerListenerPort")]
    pub r#load_balancer_listener_port: Box<i32>,
    #[builder(into)]
    #[serde(rename = "loadBalancerTargetGroup")]
    pub r#load_balancer_target_group: Box<Vec<super::super::types::ec2::GetNetworkInsightsAnalysisExplanationLoadBalancerTargetGroup>>,
    #[builder(into)]
    #[serde(rename = "loadBalancerTargetGroups")]
    pub r#load_balancer_target_groups: Box<Vec<super::super::types::ec2::GetNetworkInsightsAnalysisExplanationLoadBalancerTargetGroup>>,
    #[builder(into)]
    #[serde(rename = "loadBalancerTargetPort")]
    pub r#load_balancer_target_port: Box<i32>,
    #[builder(into)]
    #[serde(rename = "missingComponent")]
    pub r#missing_component: Box<String>,
    #[builder(into)]
    #[serde(rename = "natGateways")]
    pub r#nat_gateways: Box<Vec<super::super::types::ec2::GetNetworkInsightsAnalysisExplanationNatGateway>>,
    #[builder(into)]
    #[serde(rename = "networkInterfaces")]
    pub r#network_interfaces: Box<Vec<super::super::types::ec2::GetNetworkInsightsAnalysisExplanationNetworkInterface>>,
    #[builder(into)]
    #[serde(rename = "packetField")]
    pub r#packet_field: Box<String>,
    #[builder(into)]
    #[serde(rename = "port")]
    pub r#port: Box<i32>,
    #[builder(into)]
    #[serde(rename = "portRanges")]
    pub r#port_ranges: Box<Vec<super::super::types::ec2::GetNetworkInsightsAnalysisExplanationPortRange>>,
    #[builder(into)]
    #[serde(rename = "prefixLists")]
    pub r#prefix_lists: Box<Vec<super::super::types::ec2::GetNetworkInsightsAnalysisExplanationPrefixList>>,
    #[builder(into)]
    #[serde(rename = "protocols")]
    pub r#protocols: Box<Vec<String>>,
    #[builder(into)]
    #[serde(rename = "routeTableRoutes")]
    pub r#route_table_routes: Box<Vec<super::super::types::ec2::GetNetworkInsightsAnalysisExplanationRouteTableRoute>>,
    #[builder(into)]
    #[serde(rename = "routeTables")]
    pub r#route_tables: Box<Vec<super::super::types::ec2::GetNetworkInsightsAnalysisExplanationRouteTable>>,
    #[builder(into)]
    #[serde(rename = "securityGroup")]
    pub r#security_group: Box<Vec<super::super::types::ec2::GetNetworkInsightsAnalysisExplanationSecurityGroup>>,
    #[builder(into)]
    #[serde(rename = "securityGroupRules")]
    pub r#security_group_rules: Box<Vec<super::super::types::ec2::GetNetworkInsightsAnalysisExplanationSecurityGroupRule>>,
    #[builder(into)]
    #[serde(rename = "securityGroups")]
    pub r#security_groups: Box<Vec<super::super::types::ec2::GetNetworkInsightsAnalysisExplanationSecurityGroup>>,
    #[builder(into)]
    #[serde(rename = "sourceVpcs")]
    pub r#source_vpcs: Box<Vec<super::super::types::ec2::GetNetworkInsightsAnalysisExplanationSourceVpc>>,
    #[builder(into)]
    #[serde(rename = "state")]
    pub r#state: Box<String>,
    #[builder(into)]
    #[serde(rename = "subnetRouteTables")]
    pub r#subnet_route_tables: Box<Vec<super::super::types::ec2::GetNetworkInsightsAnalysisExplanationSubnetRouteTable>>,
    #[builder(into)]
    #[serde(rename = "subnets")]
    pub r#subnets: Box<Vec<super::super::types::ec2::GetNetworkInsightsAnalysisExplanationSubnet>>,
    #[builder(into)]
    #[serde(rename = "transitGatewayAttachments")]
    pub r#transit_gateway_attachments: Box<Vec<super::super::types::ec2::GetNetworkInsightsAnalysisExplanationTransitGatewayAttachment>>,
    #[builder(into)]
    #[serde(rename = "transitGatewayRouteTableRoutes")]
    pub r#transit_gateway_route_table_routes: Box<Vec<super::super::types::ec2::GetNetworkInsightsAnalysisExplanationTransitGatewayRouteTableRoute>>,
    #[builder(into)]
    #[serde(rename = "transitGatewayRouteTables")]
    pub r#transit_gateway_route_tables: Box<Vec<super::super::types::ec2::GetNetworkInsightsAnalysisExplanationTransitGatewayRouteTable>>,
    #[builder(into)]
    #[serde(rename = "transitGateways")]
    pub r#transit_gateways: Box<Vec<super::super::types::ec2::GetNetworkInsightsAnalysisExplanationTransitGateway>>,
    #[builder(into)]
    #[serde(rename = "vpcEndpoints")]
    pub r#vpc_endpoints: Box<Vec<super::super::types::ec2::GetNetworkInsightsAnalysisExplanationVpcEndpoint>>,
    #[builder(into)]
    #[serde(rename = "vpcPeeringConnections")]
    pub r#vpc_peering_connections: Box<Vec<super::super::types::ec2::GetNetworkInsightsAnalysisExplanationVpcPeeringConnection>>,
    #[builder(into)]
    #[serde(rename = "vpcs")]
    pub r#vpcs: Box<Vec<super::super::types::ec2::GetNetworkInsightsAnalysisExplanationVpc>>,
    #[builder(into)]
    #[serde(rename = "vpnConnections")]
    pub r#vpn_connections: Box<Vec<super::super::types::ec2::GetNetworkInsightsAnalysisExplanationVpnConnection>>,
    #[builder(into)]
    #[serde(rename = "vpnGateways")]
    pub r#vpn_gateways: Box<Vec<super::super::types::ec2::GetNetworkInsightsAnalysisExplanationVpnGateway>>,
}
