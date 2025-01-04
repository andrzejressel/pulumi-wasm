#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetGatewayRouteSpecHttp2RouteMatchHostname {
    #[builder(into)]
    #[serde(rename = "exact")]
    pub r#exact: Box<String>,
    #[builder(into)]
    #[serde(rename = "suffix")]
    pub r#suffix: Box<String>,
}
