#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct RouteSpecGrpcRouteRetryPolicy {
    /// List of gRPC retry events.
    /// Valid values: `cancelled`, `deadline-exceeded`, `internal`, `resource-exhausted`, `unavailable`.
    #[builder(into, default)]
    #[serde(rename = "grpcRetryEvents")]
    pub r#grpc_retry_events: Box<Option<Vec<String>>>,
    /// List of HTTP retry events.
    /// Valid values: `client-error` (HTTP status code 409), `gateway-error` (HTTP status codes 502, 503, and 504), `server-error` (HTTP status codes 500, 501, 502, 503, 504, 505, 506, 507, 508, 510, and 511), `stream-error` (retry on refused stream).
    #[builder(into, default)]
    #[serde(rename = "httpRetryEvents")]
    pub r#http_retry_events: Box<Option<Vec<String>>>,
    /// Maximum number of retries.
    #[builder(into)]
    #[serde(rename = "maxRetries")]
    pub r#max_retries: Box<i32>,
    /// Per-retry timeout.
    #[builder(into)]
    #[serde(rename = "perRetryTimeout")]
    pub r#per_retry_timeout: Box<super::super::types::appmesh::RouteSpecGrpcRouteRetryPolicyPerRetryTimeout>,
    /// List of TCP retry events. The only valid value is `connection-error`.
    #[builder(into, default)]
    #[serde(rename = "tcpRetryEvents")]
    pub r#tcp_retry_events: Box<Option<Vec<String>>>,
}
