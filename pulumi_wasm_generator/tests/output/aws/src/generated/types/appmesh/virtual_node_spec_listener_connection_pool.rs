#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct VirtualNodeSpecListenerConnectionPool {
    /// Connection pool information for gRPC listeners.
    #[builder(into, default)]
    #[serde(rename = "grpc")]
    pub r#grpc: Box<Option<super::super::types::appmesh::VirtualNodeSpecListenerConnectionPoolGrpc>>,
    /// Connection pool information for HTTP2 listeners.
    #[builder(into, default)]
    #[serde(rename = "http2s")]
    pub r#http_2_s: Box<Option<Vec<super::super::types::appmesh::VirtualNodeSpecListenerConnectionPoolHttp2>>>,
    /// Connection pool information for HTTP listeners.
    #[builder(into, default)]
    #[serde(rename = "https")]
    pub r#https: Box<Option<Vec<super::super::types::appmesh::VirtualNodeSpecListenerConnectionPoolHttp>>>,
    /// Connection pool information for TCP listeners.
    #[builder(into, default)]
    #[serde(rename = "tcps")]
    pub r#tcps: Box<Option<Vec<super::super::types::appmesh::VirtualNodeSpecListenerConnectionPoolTcp>>>,
}
