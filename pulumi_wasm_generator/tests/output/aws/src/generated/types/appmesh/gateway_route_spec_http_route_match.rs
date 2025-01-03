#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GatewayRouteSpecHttpRouteMatch {
    /// Client request headers to match on.
    #[builder(into, default)]
    #[serde(rename = "headers")]
    pub r#headers: Box<Option<Vec<super::super::types::appmesh::GatewayRouteSpecHttpRouteMatchHeader>>>,
    /// Host name to match on.
    #[builder(into, default)]
    #[serde(rename = "hostname")]
    pub r#hostname: Box<Option<super::super::types::appmesh::GatewayRouteSpecHttpRouteMatchHostname>>,
    /// Client request path to match on.
    #[builder(into, default)]
    #[serde(rename = "path")]
    pub r#path: Box<Option<super::super::types::appmesh::GatewayRouteSpecHttpRouteMatchPath>>,
    /// The port number to match from the request.
    #[builder(into, default)]
    #[serde(rename = "port")]
    pub r#port: Box<Option<i32>>,
    /// Path to match requests with. This parameter must always start with `/`, which by itself matches all requests to the virtual service name.
    #[builder(into, default)]
    #[serde(rename = "prefix")]
    pub r#prefix: Box<Option<String>>,
    /// Client request query parameters to match on.
    #[builder(into, default)]
    #[serde(rename = "queryParameters")]
    pub r#query_parameters: Box<Option<Vec<super::super::types::appmesh::GatewayRouteSpecHttpRouteMatchQueryParameter>>>,
}
