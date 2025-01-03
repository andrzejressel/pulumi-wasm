#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct RouteSpecHttp2Route {
    /// Action to take if a match is determined.
    #[builder(into)]
    #[serde(rename = "action")]
    pub r#action: Box<super::super::types::appmesh::RouteSpecHttp2RouteAction>,
    /// Criteria for determining an HTTP request match.
    #[builder(into)]
    #[serde(rename = "match")]
    pub r#match_: Box<super::super::types::appmesh::RouteSpecHttp2RouteMatch>,
    /// Retry policy.
    #[builder(into, default)]
    #[serde(rename = "retryPolicy")]
    pub r#retry_policy: Box<Option<super::super::types::appmesh::RouteSpecHttp2RouteRetryPolicy>>,
    /// Types of timeouts.
    #[builder(into, default)]
    #[serde(rename = "timeout")]
    pub r#timeout: Box<Option<super::super::types::appmesh::RouteSpecHttp2RouteTimeout>>,
}
