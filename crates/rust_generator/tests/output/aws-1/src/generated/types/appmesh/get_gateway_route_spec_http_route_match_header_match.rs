#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct GetGatewayRouteSpecHttpRouteMatchHeaderMatch {
    #[builder(into)]
    #[serde(rename = "exact")]
    pub r#exact: Box<String>,
    #[builder(into)]
    #[serde(rename = "prefix")]
    pub r#prefix: Box<String>,
    #[builder(into)]
    #[serde(rename = "ranges")]
    pub r#ranges: Box<Vec<super::super::types::appmesh::GetGatewayRouteSpecHttpRouteMatchHeaderMatchRange>>,
    #[builder(into)]
    #[serde(rename = "regex")]
    pub r#regex: Box<String>,
    #[builder(into)]
    #[serde(rename = "suffix")]
    pub r#suffix: Box<String>,
}
