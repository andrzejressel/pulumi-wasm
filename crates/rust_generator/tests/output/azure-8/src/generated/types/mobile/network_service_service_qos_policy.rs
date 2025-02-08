#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct NetworkServiceServiceQosPolicy {
    /// QoS Flow allocation and retention priority (ARP) level. Flows with higher priority preempt flows with lower priority, if the settings of `preemption_capability` and `preemption_vulnerability` allow it. 1 is the highest level of priority. If this field is not specified then `qos_indicator` is used to derive the ARP value. Defaults to `9`. See 3GPP TS23.501 section 5.7.2.2 for a full description of the ARP parameters.
    #[builder(into, default)]
    #[serde(rename = "allocationAndRetentionPriorityLevel")]
    pub r#allocation_and_retention_priority_level: Box<Option<i32>>,
    /// A `maximum_bit_rate` block as defined below. The Maximum Bit Rate (MBR) for all service data flows that use this PCC Rule or Service.
    #[builder(into)]
    #[serde(rename = "maximumBitRate")]
    pub r#maximum_bit_rate: Box<super::super::types::mobile::NetworkServiceServiceQosPolicyMaximumBitRate>,
    /// The Preemption Capability of a QoS Flow controls whether it can preempt another QoS Flow with a lower priority level. See 3GPP TS23.501 section 5.7.2.2 for a full description of the ARP parameters. Possible values are `NotPreempt` and `MayPreempt`,.
    #[builder(into, default)]
    #[serde(rename = "preemptionCapability")]
    pub r#preemption_capability: Box<Option<String>>,
    /// The Preemption Vulnerability of a QoS Flow controls whether it can be preempted by QoS Flow with a higher priority level. See 3GPP TS23.501 section 5.7.2.2 for a full description of the ARP parameters. Possible values are `NotPreemptable` and `Preemptable`.
    #[builder(into, default)]
    #[serde(rename = "preemptionVulnerability")]
    pub r#preemption_vulnerability: Box<Option<String>>,
    /// The QoS Indicator (5QI for 5G network /QCI for 4G net work) value identifies a set of QoS characteristics that control QoS forwarding treatment for QoS flows or EPS bearers. Recommended values: 5-9; 69-70; 79-80. Must be between `1` and `127`.
    #[builder(into, default)]
    #[serde(rename = "qosIndicator")]
    pub r#qos_indicator: Box<Option<i32>>,
}
