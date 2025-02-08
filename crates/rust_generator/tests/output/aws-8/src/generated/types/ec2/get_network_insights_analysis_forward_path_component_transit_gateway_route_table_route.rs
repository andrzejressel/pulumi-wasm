#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetNetworkInsightsAnalysisForwardPathComponentTransitGatewayRouteTableRoute {
    #[builder(into)]
    #[serde(rename = "attachmentId")]
    pub r#attachment_id: Box<String>,
    #[builder(into)]
    #[serde(rename = "destinationCidr")]
    pub r#destination_cidr: Box<String>,
    #[builder(into)]
    #[serde(rename = "prefixListId")]
    pub r#prefix_list_id: Box<String>,
    #[builder(into)]
    #[serde(rename = "resourceId")]
    pub r#resource_id: Box<String>,
    #[builder(into)]
    #[serde(rename = "resourceType")]
    pub r#resource_type: Box<String>,
    #[builder(into)]
    #[serde(rename = "routeOrigin")]
    pub r#route_origin: Box<String>,
    #[builder(into)]
    #[serde(rename = "state")]
    pub r#state: Box<String>,
}
