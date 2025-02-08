#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct GetRouteTableRoutesRoute {
    /// The CIDR used for route destination matches.
    #[builder(into)]
    #[serde(rename = "destinationCidrBlock")]
    pub r#destination_cidr_block: Box<String>,
    /// The ID of the prefix list used for destination matches.
    #[builder(into)]
    #[serde(rename = "prefixListId")]
    pub r#prefix_list_id: Box<String>,
    /// The current state of the route, can be `active`, `deleted`, `pending`, `blackhole`, `deleting`.
    #[builder(into)]
    #[serde(rename = "state")]
    pub r#state: Box<String>,
    /// The id of the transit gateway route table announcement, most of the time it is an empty string.
    #[builder(into)]
    #[serde(rename = "transitGatewayRouteTableAnnouncementId")]
    pub r#transit_gateway_route_table_announcement_id: Box<String>,
    /// The type of the route, can be `propagated` or `static`.
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: Box<String>,
}
