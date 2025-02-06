#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetRouteSpecHttp2RouteMatchQueryParameter {
    #[builder(into)]
    #[serde(rename = "matches")]
    pub r#matches: Box<Vec<super::super::types::appmesh::GetRouteSpecHttp2RouteMatchQueryParameterMatch>>,
    /// Name of the route.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
}
