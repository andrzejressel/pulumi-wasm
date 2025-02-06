#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct VirtualHubConnectionRouting {
    /// The ID of the route table associated with this Virtual Hub connection.
    #[builder(into, default)]
    #[serde(rename = "associatedRouteTableId")]
    pub r#associated_route_table_id: Box<Option<String>>,
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
    pub r#propagated_route_table: Box<Option<super::super::types::network::VirtualHubConnectionRoutingPropagatedRouteTable>>,
    /// The static VNet local route override criteria that is used to determine whether NVA in spoke VNet is bypassed for traffic with destination in spoke VNet. Possible values are `Contains` and `Equal`. Defaults to `Contains`. Changing this forces a new resource to be created.
    #[builder(into, default)]
    #[serde(rename = "staticVnetLocalRouteOverrideCriteria")]
    pub r#static_vnet_local_route_override_criteria: Box<Option<String>>,
    /// A `static_vnet_route` block as defined below.
    #[builder(into, default)]
    #[serde(rename = "staticVnetRoutes")]
    pub r#static_vnet_routes: Box<Option<Vec<super::super::types::network::VirtualHubConnectionRoutingStaticVnetRoute>>>,
}
