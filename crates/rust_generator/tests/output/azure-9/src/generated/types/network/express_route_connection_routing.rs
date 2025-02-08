#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct ExpressRouteConnectionRouting {
    /// The ID of the Virtual Hub Route Table associated with this Express Route Connection.
    #[builder(into, default)]
    #[serde(rename = "associatedRouteTableId")]
    pub r#associated_route_table_id: Box<Option<String>>,
    /// The ID of the Route Map associated with this Express Route Connection for inbound routes.
    #[builder(into, default)]
    #[serde(rename = "inboundRouteMapId")]
    pub r#inbound_route_map_id: Box<Option<String>>,
    /// The ID of the Route Map associated with this Express Route Connection for outbound routes.
    #[builder(into, default)]
    #[serde(rename = "outboundRouteMapId")]
    pub r#outbound_route_map_id: Box<Option<String>>,
    /// A `propagated_route_table` block as defined below.
    #[builder(into, default)]
    #[serde(rename = "propagatedRouteTable")]
    pub r#propagated_route_table: Box<Option<super::super::types::network::ExpressRouteConnectionRoutingPropagatedRouteTable>>,
}
