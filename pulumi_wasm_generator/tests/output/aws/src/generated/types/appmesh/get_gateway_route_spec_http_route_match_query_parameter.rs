#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetGatewayRouteSpecHttpRouteMatchQueryParameter {
    #[builder(into)]
    #[serde(rename = "matches")]
    pub r#matches: Box<Vec<super::super::types::appmesh::GetGatewayRouteSpecHttpRouteMatchQueryParameterMatch>>,
    /// Name of the gateway route.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
}
