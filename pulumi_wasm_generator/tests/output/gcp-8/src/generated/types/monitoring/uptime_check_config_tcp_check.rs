#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct UptimeCheckConfigTcpCheck {
    /// Contains information needed to add pings to a TCP check.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "pingConfig")]
    pub r#ping_config: Box<Option<super::super::types::monitoring::UptimeCheckConfigTcpCheckPingConfig>>,
    /// The port to the page to run the check against. Will be combined with host (specified within the `monitored_resource`) to construct the full URL.
    #[builder(into)]
    #[serde(rename = "port")]
    pub r#port: Box<i32>,
}
