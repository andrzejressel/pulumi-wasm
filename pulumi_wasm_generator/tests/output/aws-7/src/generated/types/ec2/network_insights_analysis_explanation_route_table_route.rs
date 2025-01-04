#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct NetworkInsightsAnalysisExplanationRouteTableRoute {
    #[builder(into, default)]
    #[serde(rename = "destinationCidr")]
    pub r#destination_cidr: Box<Option<String>>,
    #[builder(into, default)]
    #[serde(rename = "destinationPrefixListId")]
    pub r#destination_prefix_list_id: Box<Option<String>>,
    #[builder(into, default)]
    #[serde(rename = "egressOnlyInternetGatewayId")]
    pub r#egress_only_internet_gateway_id: Box<Option<String>>,
    #[builder(into, default)]
    #[serde(rename = "gatewayId")]
    pub r#gateway_id: Box<Option<String>>,
    #[builder(into, default)]
    #[serde(rename = "instanceId")]
    pub r#instance_id: Box<Option<String>>,
    #[builder(into, default)]
    #[serde(rename = "natGatewayId")]
    pub r#nat_gateway_id: Box<Option<String>>,
    #[builder(into, default)]
    #[serde(rename = "networkInterfaceId")]
    pub r#network_interface_id: Box<Option<String>>,
    #[builder(into, default)]
    #[serde(rename = "origin")]
    pub r#origin: Box<Option<String>>,
    #[builder(into, default)]
    #[serde(rename = "transitGatewayId")]
    pub r#transit_gateway_id: Box<Option<String>>,
    #[builder(into, default)]
    #[serde(rename = "vpcPeeringConnectionId")]
    pub r#vpc_peering_connection_id: Box<Option<String>>,
}
