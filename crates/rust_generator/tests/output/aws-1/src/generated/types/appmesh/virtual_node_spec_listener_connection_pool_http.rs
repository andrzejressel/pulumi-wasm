#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct VirtualNodeSpecListenerConnectionPoolHttp {
    /// Maximum number of outbound TCP connections Envoy can establish concurrently with all hosts in upstream cluster. Minimum value of `1`.
    #[builder(into)]
    #[serde(rename = "maxConnections")]
    pub r#max_connections: Box<i32>,
    /// Number of overflowing requests after `max_connections` Envoy will queue to upstream cluster. Minimum value of `1`.
    #[builder(into, default)]
    #[serde(rename = "maxPendingRequests")]
    pub r#max_pending_requests: Box<Option<i32>>,
}
