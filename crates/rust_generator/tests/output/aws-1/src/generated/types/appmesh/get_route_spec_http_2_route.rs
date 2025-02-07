#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetRouteSpecHttp2Route {
    #[builder(into)]
    #[serde(rename = "actions")]
    pub r#actions: Box<Vec<super::super::types::appmesh::GetRouteSpecHttp2RouteAction>>,
    #[builder(into)]
    #[serde(rename = "matches")]
    pub r#matches: Box<Vec<super::super::types::appmesh::GetRouteSpecHttp2RouteMatch>>,
    #[builder(into)]
    #[serde(rename = "retryPolicies")]
    pub r#retry_policies: Box<Vec<super::super::types::appmesh::GetRouteSpecHttp2RouteRetryPolicy>>,
    #[builder(into)]
    #[serde(rename = "timeouts")]
    pub r#timeouts: Box<Vec<super::super::types::appmesh::GetRouteSpecHttp2RouteTimeout>>,
}
