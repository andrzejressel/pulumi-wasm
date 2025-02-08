#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct PacketMirroringFilter {
    /// IP CIDR ranges that apply as a filter on the source (ingress) or
    /// destination (egress) IP in the IP header. Only IPv4 is supported.
    #[builder(into, default)]
    #[serde(rename = "cidrRanges")]
    pub r#cidr_ranges: Box<Option<Vec<String>>>,
    /// Direction of traffic to mirror.
    /// Default value is `BOTH`.
    /// Possible values are: `INGRESS`, `EGRESS`, `BOTH`.
    #[builder(into, default)]
    #[serde(rename = "direction")]
    pub r#direction: Box<Option<String>>,
    /// Possible IP protocols including tcp, udp, icmp and esp
    #[builder(into, default)]
    #[serde(rename = "ipProtocols")]
    pub r#ip_protocols: Box<Option<Vec<String>>>,
}
