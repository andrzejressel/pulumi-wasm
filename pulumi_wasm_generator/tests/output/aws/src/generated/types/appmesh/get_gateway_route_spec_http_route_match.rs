#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetGatewayRouteSpecHttpRouteMatch {
    #[builder(into)]
    #[serde(rename = "headers")]
    pub r#headers: Box<Vec<super::super::types::appmesh::GetGatewayRouteSpecHttpRouteMatchHeader>>,
    #[builder(into)]
    #[serde(rename = "hostnames")]
    pub r#hostnames: Box<Vec<super::super::types::appmesh::GetGatewayRouteSpecHttpRouteMatchHostname>>,
    #[builder(into)]
    #[serde(rename = "paths")]
    pub r#paths: Box<Vec<super::super::types::appmesh::GetGatewayRouteSpecHttpRouteMatchPath>>,
    #[builder(into)]
    #[serde(rename = "port")]
    pub r#port: Box<i32>,
    #[builder(into)]
    #[serde(rename = "prefix")]
    pub r#prefix: Box<String>,
    #[builder(into)]
    #[serde(rename = "queryParameters")]
    pub r#query_parameters: Box<Vec<super::super::types::appmesh::GetGatewayRouteSpecHttpRouteMatchQueryParameter>>,
}
