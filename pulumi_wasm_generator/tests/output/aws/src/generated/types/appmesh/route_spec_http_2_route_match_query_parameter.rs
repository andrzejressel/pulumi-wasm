#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct RouteSpecHttp2RouteMatchQueryParameter {
    /// The query parameter to match on.
    #[builder(into, default)]
    #[serde(rename = "match")]
    pub r#match_: Box<Option<super::super::types::appmesh::RouteSpecHttp2RouteMatchQueryParameterMatch>>,
    /// Name for the query parameter that will be matched on.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
}