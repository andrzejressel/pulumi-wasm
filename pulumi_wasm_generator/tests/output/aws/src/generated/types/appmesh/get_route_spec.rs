#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetRouteSpec {
    #[builder(into)]
    #[serde(rename = "grpcRoutes")]
    pub r#grpc_routes: Box<Vec<super::super::types::appmesh::GetRouteSpecGrpcRoute>>,
    #[builder(into)]
    #[serde(rename = "http2Routes")]
    pub r#http_2_routes: Box<Vec<super::super::types::appmesh::GetRouteSpecHttp2Route>>,
    #[builder(into)]
    #[serde(rename = "httpRoutes")]
    pub r#http_routes: Box<Vec<super::super::types::appmesh::GetRouteSpecHttpRoute>>,
    #[builder(into)]
    #[serde(rename = "priority")]
    pub r#priority: Box<i32>,
    #[builder(into)]
    #[serde(rename = "tcpRoutes")]
    pub r#tcp_routes: Box<Vec<super::super::types::appmesh::GetRouteSpecTcpRoute>>,
}