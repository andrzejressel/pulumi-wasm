#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GatewayRouteSpec {
    /// Specification of a gRPC gateway route.
    #[builder(into, default)]
    #[serde(rename = "grpcRoute")]
    pub r#grpc_route: Box<Option<super::super::types::appmesh::GatewayRouteSpecGrpcRoute>>,
    /// Specification of an HTTP/2 gateway route.
    #[builder(into, default)]
    #[serde(rename = "http2Route")]
    pub r#http_2_route: Box<Option<super::super::types::appmesh::GatewayRouteSpecHttp2Route>>,
    /// Specification of an HTTP gateway route.
    #[builder(into, default)]
    #[serde(rename = "httpRoute")]
    pub r#http_route: Box<Option<super::super::types::appmesh::GatewayRouteSpecHttpRoute>>,
    /// Priority for the gateway route, between `0` and `1000`.
    #[builder(into, default)]
    #[serde(rename = "priority")]
    pub r#priority: Box<Option<i32>>,
}
