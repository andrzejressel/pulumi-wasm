#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct NetworkInsightsAnalysisReturnPathComponentTransitGatewayRouteTableRoute {
    #[builder(into, default)]
    #[serde(rename = "attachmentId")]
    pub r#attachment_id: Box<Option<String>>,
    #[builder(into, default)]
    #[serde(rename = "destinationCidr")]
    pub r#destination_cidr: Box<Option<String>>,
    #[builder(into, default)]
    #[serde(rename = "prefixListId")]
    pub r#prefix_list_id: Box<Option<String>>,
    #[builder(into, default)]
    #[serde(rename = "resourceId")]
    pub r#resource_id: Box<Option<String>>,
    #[builder(into, default)]
    #[serde(rename = "resourceType")]
    pub r#resource_type: Box<Option<String>>,
    #[builder(into, default)]
    #[serde(rename = "routeOrigin")]
    pub r#route_origin: Box<Option<String>>,
    #[builder(into, default)]
    #[serde(rename = "state")]
    pub r#state: Box<Option<String>>,
}
