#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct VirtualGatewaySpecListenerConnectionPool {
    /// Connection pool information for gRPC listeners.
    #[builder(into, default)]
    #[serde(rename = "grpc")]
    pub r#grpc: Box<Option<super::super::types::appmesh::VirtualGatewaySpecListenerConnectionPoolGrpc>>,
    /// Connection pool information for HTTP listeners.
    #[builder(into, default)]
    #[serde(rename = "http")]
    pub r#http: Box<Option<super::super::types::appmesh::VirtualGatewaySpecListenerConnectionPoolHttp>>,
    /// Connection pool information for HTTP2 listeners.
    #[builder(into, default)]
    #[serde(rename = "http2")]
    pub r#http_2: Box<Option<super::super::types::appmesh::VirtualGatewaySpecListenerConnectionPoolHttp2>>,
}
