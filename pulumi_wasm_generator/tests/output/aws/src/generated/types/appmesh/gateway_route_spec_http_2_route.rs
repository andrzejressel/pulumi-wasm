#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GatewayRouteSpecHttp2Route {
    /// Action to take if a match is determined.
    #[builder(into)]
    #[serde(rename = "action")]
    pub r#action: Box<super::super::types::appmesh::GatewayRouteSpecHttp2RouteAction>,
    /// Criteria for determining a request match.
    #[builder(into)]
    #[serde(rename = "match")]
    pub r#match_: Box<super::super::types::appmesh::GatewayRouteSpecHttp2RouteMatch>,
}
