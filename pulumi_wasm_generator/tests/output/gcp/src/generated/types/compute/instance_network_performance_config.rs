#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct InstanceNetworkPerformanceConfig {
    /// The egress bandwidth tier to enable.
    /// Possible values: TIER_1, DEFAULT
    #[builder(into)]
    #[serde(rename = "totalEgressBandwidthTier")]
    pub r#total_egress_bandwidth_tier: Box<String>,
}
