#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct KxEnvironmentTransitGatewayConfigurationAttachmentNetworkAclConfiguration {
    /// The IPv4 network range to allow or deny, in CIDR notation. The specified CIDR block is modified to its canonical form. For example, `100.68.0.18/18` will be converted to `100.68.0.0/18`.
    #[builder(into)]
    #[serde(rename = "cidrBlock")]
    pub r#cidr_block: Box<String>,
    /// Defines the ICMP protocol that consists of the ICMP type and code. Defined below.
    #[builder(into, default)]
    #[serde(rename = "icmpTypeCode")]
    pub r#icmp_type_code: Box<Option<super::super::types::finspace::KxEnvironmentTransitGatewayConfigurationAttachmentNetworkAclConfigurationIcmpTypeCode>>,
    /// Range of ports the rule applies to. Defined below.
    #[builder(into, default)]
    #[serde(rename = "portRange")]
    pub r#port_range: Box<Option<super::super::types::finspace::KxEnvironmentTransitGatewayConfigurationAttachmentNetworkAclConfigurationPortRange>>,
    /// Protocol number. A value of `1` means all the protocols.
    #[builder(into)]
    #[serde(rename = "protocol")]
    pub r#protocol: Box<String>,
    /// Indicates whether to `allow` or `deny` the traffic that matches the rule.
    #[builder(into)]
    #[serde(rename = "ruleAction")]
    pub r#rule_action: Box<String>,
    /// Rule number for the entry. All the network ACL entries are processed in ascending order by rule number.
    #[builder(into)]
    #[serde(rename = "ruleNumber")]
    pub r#rule_number: Box<i32>,
}
