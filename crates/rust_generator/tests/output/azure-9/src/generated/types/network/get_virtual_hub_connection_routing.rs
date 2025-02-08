#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct GetVirtualHubConnectionRouting {
    /// The ID of the route table associated with this Virtual Hub connection.
    #[builder(into)]
    #[serde(rename = "associatedRouteTableId")]
    pub r#associated_route_table_id: Box<String>,
    /// The ID of the Route Map associated with this Routing Configuration for inbound learned routes.
    #[builder(into)]
    #[serde(rename = "inboundRouteMapId")]
    pub r#inbound_route_map_id: Box<String>,
    /// The ID of the Route Map associated with this Routing Configuration for outbound advertised routes.
    #[builder(into)]
    #[serde(rename = "outboundRouteMapId")]
    pub r#outbound_route_map_id: Box<String>,
    /// A `propagated_route_table` block as defined below.
    #[builder(into)]
    #[serde(rename = "propagatedRouteTables")]
    pub r#propagated_route_tables: Box<Vec<super::super::types::network::GetVirtualHubConnectionRoutingPropagatedRouteTable>>,
    /// The static VNet local route override criteria that is used to determine whether NVA in spoke VNet is bypassed for traffic with destination in spoke VNet.
    #[builder(into)]
    #[serde(rename = "staticVnetLocalRouteOverrideCriteria")]
    pub r#static_vnet_local_route_override_criteria: Box<String>,
    /// A `static_vnet_route` block as defined below.
    #[builder(into)]
    #[serde(rename = "staticVnetRoutes")]
    pub r#static_vnet_routes: Box<Vec<super::super::types::network::GetVirtualHubConnectionRoutingStaticVnetRoute>>,
}
