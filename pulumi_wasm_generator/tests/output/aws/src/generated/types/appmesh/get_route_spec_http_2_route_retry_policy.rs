#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetRouteSpecHttp2RouteRetryPolicy {
    #[builder(into)]
    #[serde(rename = "httpRetryEvents")]
    pub r#http_retry_events: Box<Vec<String>>,
    #[builder(into)]
    #[serde(rename = "maxRetries")]
    pub r#max_retries: Box<i32>,
    #[builder(into)]
    #[serde(rename = "perRetryTimeouts")]
    pub r#per_retry_timeouts: Box<Vec<super::super::types::appmesh::GetRouteSpecHttp2RouteRetryPolicyPerRetryTimeout>>,
    #[builder(into)]
    #[serde(rename = "tcpRetryEvents")]
    pub r#tcp_retry_events: Box<Vec<String>>,
}