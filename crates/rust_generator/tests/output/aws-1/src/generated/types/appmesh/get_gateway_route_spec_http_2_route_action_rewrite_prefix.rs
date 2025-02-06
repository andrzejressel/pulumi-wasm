#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetGatewayRouteSpecHttp2RouteActionRewritePrefix {
    #[builder(into)]
    #[serde(rename = "defaultPrefix")]
    pub r#default_prefix: Box<String>,
    #[builder(into)]
    #[serde(rename = "value")]
    pub r#value: Box<String>,
}
