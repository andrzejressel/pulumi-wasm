#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetNetworkServiceServiceQosPolicy {
    /// QoS Flow allocation and retention priority (ARP) level.
    #[builder(into)]
    #[serde(rename = "allocationAndRetentionPriorityLevel")]
    pub r#allocation_and_retention_priority_level: Box<i32>,
    /// A `maximum_bit_rate` block as defined below. The Maximum Bit Rate (MBR) for all service data flows that use this PCC Rule or Service.
    #[builder(into)]
    #[serde(rename = "maximumBitRates")]
    pub r#maximum_bit_rates: Box<Vec<super::super::types::mobile::GetNetworkServiceServiceQosPolicyMaximumBitRate>>,
    /// The Preemption Capability of a QoS Flow controls whether it can preempt another QoS Flow with a lower priority level. See 3GPP TS23.501 section 5.7.2.2 for a full description of the ARP parameters.
    #[builder(into)]
    #[serde(rename = "preemptionCapability")]
    pub r#preemption_capability: Box<String>,
    /// The Preemption Vulnerability of a QoS Flow controls whether it can be preempted by QoS Flow with a higher priority level. See 3GPP TS23.501 section 5.7.2.2 for a full description of the ARP parameters.
    #[builder(into)]
    #[serde(rename = "preemptionVulnerability")]
    pub r#preemption_vulnerability: Box<String>,
    /// The QoS Indicator (5QI for 5G network /QCI for 4G net work) value identifies a set of QoS characteristics that control QoS forwarding treatment for QoS flows or EPS bearers.
    #[builder(into)]
    #[serde(rename = "qosIndicator")]
    pub r#qos_indicator: Box<i32>,
}
