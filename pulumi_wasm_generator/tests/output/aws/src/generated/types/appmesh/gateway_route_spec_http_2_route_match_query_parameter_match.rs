#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GatewayRouteSpecHttp2RouteMatchQueryParameterMatch {
    /// The exact query parameter to match on.
    #[builder(into, default)]
    #[serde(rename = "exact")]
    pub r#exact: Box<Option<String>>,
}