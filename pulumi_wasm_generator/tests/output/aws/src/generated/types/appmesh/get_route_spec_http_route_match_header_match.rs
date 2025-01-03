#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetRouteSpecHttpRouteMatchHeaderMatch {
    #[builder(into)]
    #[serde(rename = "exact")]
    pub r#exact: Box<String>,
    #[builder(into)]
    #[serde(rename = "prefix")]
    pub r#prefix: Box<String>,
    #[builder(into)]
    #[serde(rename = "ranges")]
    pub r#ranges: Box<Vec<super::super::types::appmesh::GetRouteSpecHttpRouteMatchHeaderMatchRange>>,
    #[builder(into)]
    #[serde(rename = "regex")]
    pub r#regex: Box<String>,
    #[builder(into)]
    #[serde(rename = "suffix")]
    pub r#suffix: Box<String>,
}
