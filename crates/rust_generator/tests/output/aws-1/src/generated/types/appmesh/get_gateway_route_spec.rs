#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetGatewayRouteSpec {
    #[builder(into)]
    #[serde(rename = "grpcRoutes")]
    pub r#grpc_routes: Box<Vec<super::super::types::appmesh::GetGatewayRouteSpecGrpcRoute>>,
    #[builder(into)]
    #[serde(rename = "http2Routes")]
    pub r#http_2_routes: Box<Vec<super::super::types::appmesh::GetGatewayRouteSpecHttp2Route>>,
    #[builder(into)]
    #[serde(rename = "httpRoutes")]
    pub r#http_routes: Box<Vec<super::super::types::appmesh::GetGatewayRouteSpecHttpRoute>>,
    #[builder(into)]
    #[serde(rename = "priority")]
    pub r#priority: Box<i32>,
}
