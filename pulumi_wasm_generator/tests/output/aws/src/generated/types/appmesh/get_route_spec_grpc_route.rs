#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetRouteSpecGrpcRoute {
    #[builder(into)]
    #[serde(rename = "actions")]
    pub r#actions: Box<Vec<super::super::types::appmesh::GetRouteSpecGrpcRouteAction>>,
    #[builder(into)]
    #[serde(rename = "matches")]
    pub r#matches: Box<Vec<super::super::types::appmesh::GetRouteSpecGrpcRouteMatch>>,
    #[builder(into)]
    #[serde(rename = "retryPolicies")]
    pub r#retry_policies: Box<Vec<super::super::types::appmesh::GetRouteSpecGrpcRouteRetryPolicy>>,
    #[builder(into)]
    #[serde(rename = "timeouts")]
    pub r#timeouts: Box<Vec<super::super::types::appmesh::GetRouteSpecGrpcRouteTimeout>>,
}