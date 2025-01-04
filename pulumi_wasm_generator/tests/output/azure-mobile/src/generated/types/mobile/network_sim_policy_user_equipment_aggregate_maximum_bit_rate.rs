#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct NetworkSimPolicyUserEquipmentAggregateMaximumBitRate {
    /// Downlink bit rate. Must be a number followed by `Kbps`, `Mbps`, `Gbps` or `Tbps`.
    #[builder(into)]
    #[serde(rename = "downlink")]
    pub r#downlink: Box<String>,
    /// Uplink bit rate. Must be a number followed by `Kbps`, `Mbps`, `Gbps` or `Tbps`.
    #[builder(into)]
    #[serde(rename = "uplink")]
    pub r#uplink: Box<String>,
}
