#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct RegionNetworkFirewallPolicyWithRulesPredefinedRuleMatchLayer4Config {
    /// (Output)
    /// The IP protocol to which this rule applies. The protocol
    /// type is required when creating a firewall rule.
    /// This value can either be one of the following well
    /// known protocol strings (tcp, udp, icmp, esp, ah, ipip, sctp),
    /// or the IP protocol number.
    #[builder(into, default)]
    #[serde(rename = "ipProtocol")]
    pub r#ip_protocol: Box<Option<String>>,
    /// (Output)
    /// An optional list of ports to which this rule applies. This field
    /// is only applicable for UDP or TCP protocol. Each entry must be
    /// either an integer or a range. If not specified, this rule
    /// applies to connections through any port.
    /// Example inputs include: ["22"], ["80","443"], and
    /// ["12345-12349"].
    #[builder(into, default)]
    #[serde(rename = "ports")]
    pub r#ports: Box<Option<Vec<String>>>,
}
