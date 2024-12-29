#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetRouteSpecHttpRouteMatchHeader {
    #[builder(into)]
    #[serde(rename = "invert")]
    pub r#invert: Box<bool>,
    #[builder(into)]
    #[serde(rename = "matches")]
    pub r#matches: Box<Vec<super::super::types::appmesh::GetRouteSpecHttpRouteMatchHeaderMatch>>,
    /// Name of the route.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
}
