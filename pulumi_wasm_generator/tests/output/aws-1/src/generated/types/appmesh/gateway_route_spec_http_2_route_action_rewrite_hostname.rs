#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GatewayRouteSpecHttp2RouteActionRewriteHostname {
    /// Default target host name to write to. Valid values: `ENABLED`, `DISABLED`.
    #[builder(into)]
    #[serde(rename = "defaultTargetHostname")]
    pub r#default_target_hostname: Box<String>,
}
