#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct BlockchainNodesEthereumDetailsAdditionalEndpoint {
    /// The assigned URL for the node's Beacon API endpoint.
    #[builder(into, default)]
    #[serde(rename = "beaconApiEndpoint")]
    pub r#beacon_api_endpoint: Box<Option<String>>,
    /// The assigned URL for the node's Beacon Prometheus metrics endpoint.
    #[builder(into, default)]
    #[serde(rename = "beaconPrometheusMetricsApiEndpoint")]
    pub r#beacon_prometheus_metrics_api_endpoint: Box<Option<String>>,
    /// The assigned URL for the node's execution client's Prometheus metrics endpoint.
    #[builder(into, default)]
    #[serde(rename = "executionClientPrometheusMetricsApiEndpoint")]
    pub r#execution_client_prometheus_metrics_api_endpoint: Box<Option<String>>,
}
