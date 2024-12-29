#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetGatewayRouteSpecHttpRouteAction {
    #[builder(into)]
    #[serde(rename = "rewrites")]
    pub r#rewrites: Box<Vec<super::super::types::appmesh::GetGatewayRouteSpecHttpRouteActionRewrite>>,
    #[builder(into)]
    #[serde(rename = "targets")]
    pub r#targets: Box<Vec<super::super::types::appmesh::GetGatewayRouteSpecHttpRouteActionTarget>>,
}
