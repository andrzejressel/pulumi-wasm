#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetClusterNodePoolNetworkConfigNetworkPerformanceConfig {
    /// Specifies the total network bandwidth tier for the NodePool.
    #[builder(into)]
    #[serde(rename = "totalEgressBandwidthTier")]
    pub r#total_egress_bandwidth_tier: Box<String>,
}
