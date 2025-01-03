#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GatewayRouteSpecHttp2RouteMatchHeader {
    /// If `true`, the match is on the opposite of the `match` method and value. Default is `false`.
    #[builder(into, default)]
    #[serde(rename = "invert")]
    pub r#invert: Box<Option<bool>>,
    /// Method and value to match the header value sent with a request. Specify one match method.
    #[builder(into, default)]
    #[serde(rename = "match")]
    pub r#match_: Box<Option<super::super::types::appmesh::GatewayRouteSpecHttp2RouteMatchHeaderMatch>>,
    /// Name for the HTTP header in the client request that will be matched on.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
}
