#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GatewayRouteSpecHttpRouteMatchHostname {
    /// Exact host name to match on.
    #[builder(into, default)]
    #[serde(rename = "exact")]
    pub r#exact: Box<Option<String>>,
    /// Specified ending characters of the host name to match on.
    #[builder(into, default)]
    #[serde(rename = "suffix")]
    pub r#suffix: Box<Option<String>>,
}
