#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct PointToPointVpnGatewayConnectionConfigurationRoute {
    /// The Virtual Hub Route Table resource id associated with this Routing Configuration.
    #[builder(into)]
    #[serde(rename = "associatedRouteTableId")]
    pub r#associated_route_table_id: Box<String>,
    /// The resource ID of the Route Map associated with this Routing Configuration for inbound learned routes.
    #[builder(into, default)]
    #[serde(rename = "inboundRouteMapId")]
    pub r#inbound_route_map_id: Box<Option<String>>,
    /// The resource ID of the Route Map associated with this Routing Configuration for outbound advertised routes.
    #[builder(into, default)]
    #[serde(rename = "outboundRouteMapId")]
    pub r#outbound_route_map_id: Box<Option<String>>,
    /// A `propagated_route_table` block as defined below.
    #[builder(into, default)]
    #[serde(rename = "propagatedRouteTable")]
    pub r#propagated_route_table: Box<Option<super::super::types::network::PointToPointVpnGatewayConnectionConfigurationRoutePropagatedRouteTable>>,
}
