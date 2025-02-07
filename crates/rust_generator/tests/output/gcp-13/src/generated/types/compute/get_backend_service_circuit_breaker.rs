#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetBackendServiceCircuitBreaker {
    /// The timeout for new network connections to hosts.
    #[builder(into)]
    #[serde(rename = "connectTimeouts")]
    pub r#connect_timeouts: Box<Vec<super::super::types::compute::GetBackendServiceCircuitBreakerConnectTimeout>>,
    /// The maximum number of connections to the backend cluster.
    /// Defaults to 1024.
    #[builder(into)]
    #[serde(rename = "maxConnections")]
    pub r#max_connections: Box<i32>,
    /// The maximum number of pending requests to the backend cluster.
    /// Defaults to 1024.
    #[builder(into)]
    #[serde(rename = "maxPendingRequests")]
    pub r#max_pending_requests: Box<i32>,
    /// The maximum number of parallel requests to the backend cluster.
    /// Defaults to 1024.
    #[builder(into)]
    #[serde(rename = "maxRequests")]
    pub r#max_requests: Box<i32>,
    /// Maximum requests for a single backend connection. This parameter
    /// is respected by both the HTTP/1.1 and HTTP/2 implementations. If
    /// not specified, there is no limit. Setting this parameter to 1
    /// will effectively disable keep alive.
    #[builder(into)]
    #[serde(rename = "maxRequestsPerConnection")]
    pub r#max_requests_per_connection: Box<i32>,
    /// The maximum number of parallel retries to the backend cluster.
    /// Defaults to 3.
    #[builder(into)]
    #[serde(rename = "maxRetries")]
    pub r#max_retries: Box<i32>,
}
