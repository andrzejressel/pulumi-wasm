#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetRouteSpecHttpRouteMatch {
    #[builder(into)]
    #[serde(rename = "headers")]
    pub r#headers: Box<Vec<super::super::types::appmesh::GetRouteSpecHttpRouteMatchHeader>>,
    #[builder(into)]
    #[serde(rename = "method")]
    pub r#method: Box<String>,
    #[builder(into)]
    #[serde(rename = "paths")]
    pub r#paths: Box<Vec<super::super::types::appmesh::GetRouteSpecHttpRouteMatchPath>>,
    #[builder(into)]
    #[serde(rename = "port")]
    pub r#port: Box<i32>,
    #[builder(into)]
    #[serde(rename = "prefix")]
    pub r#prefix: Box<String>,
    #[builder(into)]
    #[serde(rename = "queryParameters")]
    pub r#query_parameters: Box<Vec<super::super::types::appmesh::GetRouteSpecHttpRouteMatchQueryParameter>>,
    #[builder(into)]
    #[serde(rename = "scheme")]
    pub r#scheme: Box<String>,
}
