#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct NetworkInsightsAnalysisExplanation {
    #[builder(into, default)]
    #[serde(rename = "aclRules")]
    pub r#acl_rules: Box<Option<Vec<super::super::types::ec2::NetworkInsightsAnalysisExplanationAclRule>>>,
    #[builder(into, default)]
    #[serde(rename = "acls")]
    pub r#acls: Box<Option<Vec<super::super::types::ec2::NetworkInsightsAnalysisExplanationAcl>>>,
    #[builder(into, default)]
    #[serde(rename = "address")]
    pub r#address: Box<Option<String>>,
    #[builder(into, default)]
    #[serde(rename = "addresses")]
    pub r#addresses: Box<Option<Vec<String>>>,
    #[builder(into, default)]
    #[serde(rename = "attachedTos")]
    pub r#attached_tos: Box<Option<Vec<super::super::types::ec2::NetworkInsightsAnalysisExplanationAttachedTo>>>,
    #[builder(into, default)]
    #[serde(rename = "availabilityZones")]
    pub r#availability_zones: Box<Option<Vec<String>>>,
    #[builder(into, default)]
    #[serde(rename = "cidrs")]
    pub r#cidrs: Box<Option<Vec<String>>>,
    #[builder(into, default)]
    #[serde(rename = "classicLoadBalancerListeners")]
    pub r#classic_load_balancer_listeners: Box<Option<Vec<super::super::types::ec2::NetworkInsightsAnalysisExplanationClassicLoadBalancerListener>>>,
    #[builder(into, default)]
    #[serde(rename = "components")]
    pub r#components: Box<Option<Vec<super::super::types::ec2::NetworkInsightsAnalysisExplanationComponent>>>,
    #[builder(into, default)]
    #[serde(rename = "customerGateways")]
    pub r#customer_gateways: Box<Option<Vec<super::super::types::ec2::NetworkInsightsAnalysisExplanationCustomerGateway>>>,
    #[builder(into, default)]
    #[serde(rename = "destinationVpcs")]
    pub r#destination_vpcs: Box<Option<Vec<super::super::types::ec2::NetworkInsightsAnalysisExplanationDestinationVpc>>>,
    #[builder(into, default)]
    #[serde(rename = "destinations")]
    pub r#destinations: Box<Option<Vec<super::super::types::ec2::NetworkInsightsAnalysisExplanationDestination>>>,
    #[builder(into, default)]
    #[serde(rename = "direction")]
    pub r#direction: Box<Option<String>>,
    #[builder(into, default)]
    #[serde(rename = "elasticLoadBalancerListeners")]
    pub r#elastic_load_balancer_listeners: Box<Option<Vec<super::super::types::ec2::NetworkInsightsAnalysisExplanationElasticLoadBalancerListener>>>,
    #[builder(into, default)]
    #[serde(rename = "explanationCode")]
    pub r#explanation_code: Box<Option<String>>,
    #[builder(into, default)]
    #[serde(rename = "ingressRouteTables")]
    pub r#ingress_route_tables: Box<Option<Vec<super::super::types::ec2::NetworkInsightsAnalysisExplanationIngressRouteTable>>>,
    #[builder(into, default)]
    #[serde(rename = "internetGateways")]
    pub r#internet_gateways: Box<Option<Vec<super::super::types::ec2::NetworkInsightsAnalysisExplanationInternetGateway>>>,
    #[builder(into, default)]
    #[serde(rename = "loadBalancerArn")]
    pub r#load_balancer_arn: Box<Option<String>>,
    #[builder(into, default)]
    #[serde(rename = "loadBalancerListenerPort")]
    pub r#load_balancer_listener_port: Box<Option<i32>>,
    #[builder(into, default)]
    #[serde(rename = "loadBalancerTargetGroup")]
    pub r#load_balancer_target_group: Box<Option<Vec<super::super::types::ec2::NetworkInsightsAnalysisExplanationLoadBalancerTargetGroup>>>,
    #[builder(into, default)]
    #[serde(rename = "loadBalancerTargetGroups")]
    pub r#load_balancer_target_groups: Box<Option<Vec<super::super::types::ec2::NetworkInsightsAnalysisExplanationLoadBalancerTargetGroup>>>,
    #[builder(into, default)]
    #[serde(rename = "loadBalancerTargetPort")]
    pub r#load_balancer_target_port: Box<Option<i32>>,
    #[builder(into, default)]
    #[serde(rename = "missingComponent")]
    pub r#missing_component: Box<Option<String>>,
    #[builder(into, default)]
    #[serde(rename = "natGateways")]
    pub r#nat_gateways: Box<Option<Vec<super::super::types::ec2::NetworkInsightsAnalysisExplanationNatGateway>>>,
    #[builder(into, default)]
    #[serde(rename = "networkInterfaces")]
    pub r#network_interfaces: Box<Option<Vec<super::super::types::ec2::NetworkInsightsAnalysisExplanationNetworkInterface>>>,
    #[builder(into, default)]
    #[serde(rename = "packetField")]
    pub r#packet_field: Box<Option<String>>,
    #[builder(into, default)]
    #[serde(rename = "port")]
    pub r#port: Box<Option<i32>>,
    #[builder(into, default)]
    #[serde(rename = "portRanges")]
    pub r#port_ranges: Box<Option<Vec<super::super::types::ec2::NetworkInsightsAnalysisExplanationPortRange>>>,
    #[builder(into, default)]
    #[serde(rename = "prefixLists")]
    pub r#prefix_lists: Box<Option<Vec<super::super::types::ec2::NetworkInsightsAnalysisExplanationPrefixList>>>,
    #[builder(into, default)]
    #[serde(rename = "protocols")]
    pub r#protocols: Box<Option<Vec<String>>>,
    #[builder(into, default)]
    #[serde(rename = "routeTableRoutes")]
    pub r#route_table_routes: Box<Option<Vec<super::super::types::ec2::NetworkInsightsAnalysisExplanationRouteTableRoute>>>,
    #[builder(into, default)]
    #[serde(rename = "routeTables")]
    pub r#route_tables: Box<Option<Vec<super::super::types::ec2::NetworkInsightsAnalysisExplanationRouteTable>>>,
    #[builder(into, default)]
    #[serde(rename = "securityGroup")]
    pub r#security_group: Box<Option<Vec<super::super::types::ec2::NetworkInsightsAnalysisExplanationSecurityGroup>>>,
    #[builder(into, default)]
    #[serde(rename = "securityGroupRules")]
    pub r#security_group_rules: Box<Option<Vec<super::super::types::ec2::NetworkInsightsAnalysisExplanationSecurityGroupRule>>>,
    #[builder(into, default)]
    #[serde(rename = "securityGroups")]
    pub r#security_groups: Box<Option<Vec<super::super::types::ec2::NetworkInsightsAnalysisExplanationSecurityGroup>>>,
    #[builder(into, default)]
    #[serde(rename = "sourceVpcs")]
    pub r#source_vpcs: Box<Option<Vec<super::super::types::ec2::NetworkInsightsAnalysisExplanationSourceVpc>>>,
    #[builder(into, default)]
    #[serde(rename = "state")]
    pub r#state: Box<Option<String>>,
    #[builder(into, default)]
    #[serde(rename = "subnetRouteTables")]
    pub r#subnet_route_tables: Box<Option<Vec<super::super::types::ec2::NetworkInsightsAnalysisExplanationSubnetRouteTable>>>,
    #[builder(into, default)]
    #[serde(rename = "subnets")]
    pub r#subnets: Box<Option<Vec<super::super::types::ec2::NetworkInsightsAnalysisExplanationSubnet>>>,
    #[builder(into, default)]
    #[serde(rename = "transitGatewayAttachments")]
    pub r#transit_gateway_attachments: Box<Option<Vec<super::super::types::ec2::NetworkInsightsAnalysisExplanationTransitGatewayAttachment>>>,
    #[builder(into, default)]
    #[serde(rename = "transitGatewayRouteTableRoutes")]
    pub r#transit_gateway_route_table_routes: Box<Option<Vec<super::super::types::ec2::NetworkInsightsAnalysisExplanationTransitGatewayRouteTableRoute>>>,
    #[builder(into, default)]
    #[serde(rename = "transitGatewayRouteTables")]
    pub r#transit_gateway_route_tables: Box<Option<Vec<super::super::types::ec2::NetworkInsightsAnalysisExplanationTransitGatewayRouteTable>>>,
    #[builder(into, default)]
    #[serde(rename = "transitGateways")]
    pub r#transit_gateways: Box<Option<Vec<super::super::types::ec2::NetworkInsightsAnalysisExplanationTransitGateway>>>,
    #[builder(into, default)]
    #[serde(rename = "vpcEndpoints")]
    pub r#vpc_endpoints: Box<Option<Vec<super::super::types::ec2::NetworkInsightsAnalysisExplanationVpcEndpoint>>>,
    #[builder(into, default)]
    #[serde(rename = "vpcPeeringConnections")]
    pub r#vpc_peering_connections: Box<Option<Vec<super::super::types::ec2::NetworkInsightsAnalysisExplanationVpcPeeringConnection>>>,
    #[builder(into, default)]
    #[serde(rename = "vpcs")]
    pub r#vpcs: Box<Option<Vec<super::super::types::ec2::NetworkInsightsAnalysisExplanationVpc>>>,
    #[builder(into, default)]
    #[serde(rename = "vpnConnections")]
    pub r#vpn_connections: Box<Option<Vec<super::super::types::ec2::NetworkInsightsAnalysisExplanationVpnConnection>>>,
    #[builder(into, default)]
    #[serde(rename = "vpnGateways")]
    pub r#vpn_gateways: Box<Option<Vec<super::super::types::ec2::NetworkInsightsAnalysisExplanationVpnGateway>>>,
}
