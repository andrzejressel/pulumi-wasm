#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GatewayRouteSpecHttpRouteActionRewrite {
    /// Host name to rewrite.
    #[builder(into, default)]
    #[serde(rename = "hostname")]
    pub r#hostname: Box<Option<super::super::types::appmesh::GatewayRouteSpecHttpRouteActionRewriteHostname>>,
    /// Exact path to rewrite.
    #[builder(into, default)]
    #[serde(rename = "path")]
    pub r#path: Box<Option<super::super::types::appmesh::GatewayRouteSpecHttpRouteActionRewritePath>>,
    /// Specified beginning characters to rewrite.
    #[builder(into, default)]
    #[serde(rename = "prefix")]
    pub r#prefix: Box<Option<super::super::types::appmesh::GatewayRouteSpecHttpRouteActionRewritePrefix>>,
}
