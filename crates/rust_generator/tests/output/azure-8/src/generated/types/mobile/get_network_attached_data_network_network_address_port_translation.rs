#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct GetNetworkAttachedDataNetworkNetworkAddressPortTranslation {
    #[builder(into)]
    #[serde(rename = "icmpPinholeTimeoutInSeconds")]
    pub r#icmp_pinhole_timeout_in_seconds: Box<i32>,
    #[builder(into, default)]
    #[serde(rename = "pinholeMaximumNumber")]
    pub r#pinhole_maximum_number: Box<Option<i32>>,
    /// A `port_range` block as defined below.
    #[builder(into)]
    #[serde(rename = "portRanges")]
    pub r#port_ranges: Box<Vec<super::super::types::mobile::GetNetworkAttachedDataNetworkNetworkAddressPortTranslationPortRange>>,
    #[builder(into)]
    #[serde(rename = "tcpPinholeTimeoutInSeconds")]
    pub r#tcp_pinhole_timeout_in_seconds: Box<i32>,
    /// Minimum time in seconds that will pass before a TCP port that was used by a closed pinhole can be reused.
    #[builder(into)]
    #[serde(rename = "tcpPortReuseMinimumHoldTimeInSeconds")]
    pub r#tcp_port_reuse_minimum_hold_time_in_seconds: Box<i32>,
    #[builder(into)]
    #[serde(rename = "udpPinholeTimeoutInSeconds")]
    pub r#udp_pinhole_timeout_in_seconds: Box<i32>,
    /// Minimum time in seconds that will pass before a UDP port that was used by a closed pinhole can be reused.
    #[builder(into)]
    #[serde(rename = "udpPortReuseMinimumHoldTimeInSeconds")]
    pub r#udp_port_reuse_minimum_hold_time_in_seconds: Box<i32>,
}
