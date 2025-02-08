#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct NetworkSimPolicySliceDataNetwork {
    /// Allowed session types in addition to the default session type. Must not duplicate the default session type. Possible values are `IPv4` and `IPv6`.
    #[builder(into, default)]
    #[serde(rename = "additionalAllowedSessionTypes")]
    pub r#additional_allowed_session_types: Box<Option<Vec<String>>>,
    /// Default QoS Flow allocation and retention priority (ARP) level. Flows with higher priority preempt flows with lower priority, if the settings of `preemption_capability` and `preemption_vulnerability` allow it. `1` is the highest level of priority. If this field is not specified then `qos_indicator` is used to derive the ARP value. See 3GPP TS23.501 section 5.7.2.2 for a full description of the ARP parameters.
    #[builder(into, default)]
    #[serde(rename = "allocationAndRetentionPriorityLevel")]
    pub r#allocation_and_retention_priority_level: Box<Option<i32>>,
    /// An array of IDs of services that can be used as part of this SIM policy. The array must not contain duplicate items and must contain at least one item.
    #[builder(into)]
    #[serde(rename = "allowedServicesIds")]
    pub r#allowed_services_ids: Box<Vec<String>>,
    /// The ID of Mobile Network Data Network which these settings apply to.
    #[builder(into)]
    #[serde(rename = "dataNetworkId")]
    pub r#data_network_id: Box<String>,
    /// The default PDU session type, which is used if the user equipment does not request a specific session type. Possible values are `IPv4` and `IPv6`. Defaults to `IPv4`.
    #[builder(into, default)]
    #[serde(rename = "defaultSessionType")]
    pub r#default_session_type: Box<Option<String>>,
    /// The maximum number of downlink packets to buffer at the user plane for High Latency Communication - Extended Buffering. Defaults to `10`, Must be at least `0`, See 3GPP TS29.272 v15.10.0 section 7.3.188 for a full description. This maximum is not guaranteed because there is a internal limit on buffered packets across all PDU sessions.
    #[builder(into, default)]
    #[serde(rename = "maxBufferedPackets")]
    pub r#max_buffered_packets: Box<Option<i32>>,
    /// The Preemption Capability of a QoS Flow, it controls whether it can preempt another QoS Flow with a lower priority level. See 3GPP TS23.501 section 5.7.2.2 for a full description of the ARP parameters. Possible values are `NotPreempt` and `MayPreempt`, Defaults to `NotPreempt`.
    #[builder(into, default)]
    #[serde(rename = "preemptionCapability")]
    pub r#preemption_capability: Box<Option<String>>,
    /// The Preemption Vulnerability of a QoS Flow, it controls whether it can be preempted by QoS Flow with a higher priority level. See 3GPP TS23.501 section 5.7.2.2 for a full description of the ARP parameters. Possible values are `NotPreemptable` and `Preemptable`. Defaults to `NotPreemptable`.
    #[builder(into, default)]
    #[serde(rename = "preemptionVulnerability")]
    pub r#preemption_vulnerability: Box<Option<String>>,
    /// The QoS Indicator (5QI for 5G network /QCI for 4G net work) value identifies a set of QoS characteristics, it controls QoS forwarding treatment for QoS flows or EPS bearers. Recommended values: 5-9; 69-70; 79-80. Must be between `1` and `127`.
    #[builder(into)]
    #[serde(rename = "qosIndicator")]
    pub r#qos_indicator: Box<i32>,
    /// A `session_aggregate_maximum_bit_rate` block as defined below.
    #[builder(into)]
    #[serde(rename = "sessionAggregateMaximumBitRate")]
    pub r#session_aggregate_maximum_bit_rate: Box<super::super::types::mobile::NetworkSimPolicySliceDataNetworkSessionAggregateMaximumBitRate>,
}
