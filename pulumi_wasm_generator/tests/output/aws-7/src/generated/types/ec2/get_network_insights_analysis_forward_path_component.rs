#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetNetworkInsightsAnalysisForwardPathComponent {
    #[builder(into)]
    #[serde(rename = "aclRules")]
    pub r#acl_rules: Box<Vec<super::super::types::ec2::GetNetworkInsightsAnalysisForwardPathComponentAclRule>>,
    #[builder(into)]
    #[serde(rename = "additionalDetails")]
    pub r#additional_details: Box<Vec<super::super::types::ec2::GetNetworkInsightsAnalysisForwardPathComponentAdditionalDetail>>,
    #[builder(into)]
    #[serde(rename = "attachedTos")]
    pub r#attached_tos: Box<Vec<super::super::types::ec2::GetNetworkInsightsAnalysisForwardPathComponentAttachedTo>>,
    #[builder(into)]
    #[serde(rename = "components")]
    pub r#components: Box<Vec<super::super::types::ec2::GetNetworkInsightsAnalysisForwardPathComponentComponent>>,
    #[builder(into)]
    #[serde(rename = "destinationVpcs")]
    pub r#destination_vpcs: Box<Vec<super::super::types::ec2::GetNetworkInsightsAnalysisForwardPathComponentDestinationVpc>>,
    #[builder(into)]
    #[serde(rename = "inboundHeaders")]
    pub r#inbound_headers: Box<Vec<super::super::types::ec2::GetNetworkInsightsAnalysisForwardPathComponentInboundHeader>>,
    #[builder(into)]
    #[serde(rename = "outboundHeaders")]
    pub r#outbound_headers: Box<Vec<super::super::types::ec2::GetNetworkInsightsAnalysisForwardPathComponentOutboundHeader>>,
    #[builder(into)]
    #[serde(rename = "routeTableRoutes")]
    pub r#route_table_routes: Box<Vec<super::super::types::ec2::GetNetworkInsightsAnalysisForwardPathComponentRouteTableRoute>>,
    #[builder(into)]
    #[serde(rename = "securityGroupRules")]
    pub r#security_group_rules: Box<Vec<super::super::types::ec2::GetNetworkInsightsAnalysisForwardPathComponentSecurityGroupRule>>,
    #[builder(into)]
    #[serde(rename = "sequenceNumber")]
    pub r#sequence_number: Box<i32>,
    #[builder(into)]
    #[serde(rename = "sourceVpcs")]
    pub r#source_vpcs: Box<Vec<super::super::types::ec2::GetNetworkInsightsAnalysisForwardPathComponentSourceVpc>>,
    #[builder(into)]
    #[serde(rename = "subnets")]
    pub r#subnets: Box<Vec<super::super::types::ec2::GetNetworkInsightsAnalysisForwardPathComponentSubnet>>,
    #[builder(into)]
    #[serde(rename = "transitGatewayRouteTableRoutes")]
    pub r#transit_gateway_route_table_routes: Box<Vec<super::super::types::ec2::GetNetworkInsightsAnalysisForwardPathComponentTransitGatewayRouteTableRoute>>,
    #[builder(into)]
    #[serde(rename = "transitGateways")]
    pub r#transit_gateways: Box<Vec<super::super::types::ec2::GetNetworkInsightsAnalysisForwardPathComponentTransitGateway>>,
    #[builder(into)]
    #[serde(rename = "vpcs")]
    pub r#vpcs: Box<Vec<super::super::types::ec2::GetNetworkInsightsAnalysisForwardPathComponentVpc>>,
}
