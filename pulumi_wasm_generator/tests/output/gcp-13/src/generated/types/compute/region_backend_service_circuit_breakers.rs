#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct RegionBackendServiceCircuitBreakers {
    /// The timeout for new network connections to hosts.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "connectTimeout")]
    pub r#connect_timeout: Box<Option<super::super::types::compute::RegionBackendServiceCircuitBreakersConnectTimeout>>,
    /// The maximum number of connections to the backend cluster.
    /// Defaults to 1024.
    #[builder(into, default)]
    #[serde(rename = "maxConnections")]
    pub r#max_connections: Box<Option<i32>>,
    /// The maximum number of pending requests to the backend cluster.
    /// Defaults to 1024.
    #[builder(into, default)]
    #[serde(rename = "maxPendingRequests")]
    pub r#max_pending_requests: Box<Option<i32>>,
    /// The maximum number of parallel requests to the backend cluster.
    /// Defaults to 1024.
    #[builder(into, default)]
    #[serde(rename = "maxRequests")]
    pub r#max_requests: Box<Option<i32>>,
    /// Maximum requests for a single backend connection. This parameter
    /// is respected by both the HTTP/1.1 and HTTP/2 implementations. If
    /// not specified, there is no limit. Setting this parameter to 1
    /// will effectively disable keep alive.
    #[builder(into, default)]
    #[serde(rename = "maxRequestsPerConnection")]
    pub r#max_requests_per_connection: Box<Option<i32>>,
    /// The maximum number of parallel retries to the backend cluster.
    /// Defaults to 3.
    #[builder(into, default)]
    #[serde(rename = "maxRetries")]
    pub r#max_retries: Box<Option<i32>>,
}
