#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetRouteSpecGrpcRouteRetryPolicy {
    #[builder(into)]
    #[serde(rename = "grpcRetryEvents")]
    pub r#grpc_retry_events: Box<Vec<String>>,
    #[builder(into)]
    #[serde(rename = "httpRetryEvents")]
    pub r#http_retry_events: Box<Vec<String>>,
    #[builder(into)]
    #[serde(rename = "maxRetries")]
    pub r#max_retries: Box<i32>,
    #[builder(into)]
    #[serde(rename = "perRetryTimeouts")]
    pub r#per_retry_timeouts: Box<Vec<super::super::types::appmesh::GetRouteSpecGrpcRouteRetryPolicyPerRetryTimeout>>,
    #[builder(into)]
    #[serde(rename = "tcpRetryEvents")]
    pub r#tcp_retry_events: Box<Vec<String>>,
}
