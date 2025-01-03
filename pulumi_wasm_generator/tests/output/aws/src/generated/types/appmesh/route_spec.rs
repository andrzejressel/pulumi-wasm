#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct RouteSpec {
    /// GRPC routing information for the route.
    #[builder(into, default)]
    #[serde(rename = "grpcRoute")]
    pub r#grpc_route: Box<Option<super::super::types::appmesh::RouteSpecGrpcRoute>>,
    /// HTTP/2 routing information for the route.
    #[builder(into, default)]
    #[serde(rename = "http2Route")]
    pub r#http_2_route: Box<Option<super::super::types::appmesh::RouteSpecHttp2Route>>,
    /// HTTP routing information for the route.
    #[builder(into, default)]
    #[serde(rename = "httpRoute")]
    pub r#http_route: Box<Option<super::super::types::appmesh::RouteSpecHttpRoute>>,
    /// Priority for the route, between `0` and `1000`.
    /// Routes are matched based on the specified value, where `0` is the highest priority.
    #[builder(into, default)]
    #[serde(rename = "priority")]
    pub r#priority: Box<Option<i32>>,
    /// TCP routing information for the route.
    #[builder(into, default)]
    #[serde(rename = "tcpRoute")]
    pub r#tcp_route: Box<Option<super::super::types::appmesh::RouteSpecTcpRoute>>,
}
