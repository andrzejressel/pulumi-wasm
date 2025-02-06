#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct NetworkInsightsAnalysisReturnPathComponent {
    #[builder(into, default)]
    #[serde(rename = "aclRules")]
    pub r#acl_rules: Box<Option<Vec<super::super::types::ec2::NetworkInsightsAnalysisReturnPathComponentAclRule>>>,
    #[builder(into, default)]
    #[serde(rename = "additionalDetails")]
    pub r#additional_details: Box<Option<Vec<super::super::types::ec2::NetworkInsightsAnalysisReturnPathComponentAdditionalDetail>>>,
    #[builder(into, default)]
    #[serde(rename = "attachedTos")]
    pub r#attached_tos: Box<Option<Vec<super::super::types::ec2::NetworkInsightsAnalysisReturnPathComponentAttachedTo>>>,
    #[builder(into, default)]
    #[serde(rename = "components")]
    pub r#components: Box<Option<Vec<super::super::types::ec2::NetworkInsightsAnalysisReturnPathComponentComponent>>>,
    #[builder(into, default)]
    #[serde(rename = "destinationVpcs")]
    pub r#destination_vpcs: Box<Option<Vec<super::super::types::ec2::NetworkInsightsAnalysisReturnPathComponentDestinationVpc>>>,
    #[builder(into, default)]
    #[serde(rename = "inboundHeaders")]
    pub r#inbound_headers: Box<Option<Vec<super::super::types::ec2::NetworkInsightsAnalysisReturnPathComponentInboundHeader>>>,
    #[builder(into, default)]
    #[serde(rename = "outboundHeaders")]
    pub r#outbound_headers: Box<Option<Vec<super::super::types::ec2::NetworkInsightsAnalysisReturnPathComponentOutboundHeader>>>,
    #[builder(into, default)]
    #[serde(rename = "routeTableRoutes")]
    pub r#route_table_routes: Box<Option<Vec<super::super::types::ec2::NetworkInsightsAnalysisReturnPathComponentRouteTableRoute>>>,
    #[builder(into, default)]
    #[serde(rename = "securityGroupRules")]
    pub r#security_group_rules: Box<Option<Vec<super::super::types::ec2::NetworkInsightsAnalysisReturnPathComponentSecurityGroupRule>>>,
    #[builder(into, default)]
    #[serde(rename = "sequenceNumber")]
    pub r#sequence_number: Box<Option<i32>>,
    #[builder(into, default)]
    #[serde(rename = "sourceVpcs")]
    pub r#source_vpcs: Box<Option<Vec<super::super::types::ec2::NetworkInsightsAnalysisReturnPathComponentSourceVpc>>>,
    #[builder(into, default)]
    #[serde(rename = "subnets")]
    pub r#subnets: Box<Option<Vec<super::super::types::ec2::NetworkInsightsAnalysisReturnPathComponentSubnet>>>,
    #[builder(into, default)]
    #[serde(rename = "transitGatewayRouteTableRoutes")]
    pub r#transit_gateway_route_table_routes: Box<Option<Vec<super::super::types::ec2::NetworkInsightsAnalysisReturnPathComponentTransitGatewayRouteTableRoute>>>,
    #[builder(into, default)]
    #[serde(rename = "transitGateways")]
    pub r#transit_gateways: Box<Option<Vec<super::super::types::ec2::NetworkInsightsAnalysisReturnPathComponentTransitGateway>>>,
    #[builder(into, default)]
    #[serde(rename = "vpcs")]
    pub r#vpcs: Box<Option<Vec<super::super::types::ec2::NetworkInsightsAnalysisReturnPathComponentVpc>>>,
}
