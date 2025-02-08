#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetGatewayRouteSpecHttp2RouteMatchHeader {
    #[builder(into)]
    #[serde(rename = "invert")]
    pub r#invert: Box<bool>,
    #[builder(into)]
    #[serde(rename = "matches")]
    pub r#matches: Box<Vec<super::super::types::appmesh::GetGatewayRouteSpecHttp2RouteMatchHeaderMatch>>,
    /// Name of the gateway route.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
}
