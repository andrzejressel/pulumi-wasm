#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct RegionSecurityPolicyRuleNetworkMatch {
    /// Destination IPv4/IPv6 addresses or CIDR prefixes, in standard text format.
    #[builder(into, default)]
    #[serde(rename = "destIpRanges")]
    pub r#dest_ip_ranges: Box<Option<Vec<String>>>,
    /// Destination port numbers for TCP/UDP/SCTP. Each element can be a 16-bit unsigned decimal number (e.g. "80") or range (e.g. "0-1023").
    #[builder(into, default)]
    #[serde(rename = "destPorts")]
    pub r#dest_ports: Box<Option<Vec<String>>>,
    /// IPv4 protocol / IPv6 next header (after extension headers). Each element can be an 8-bit unsigned decimal number (e.g. "6"), range (e.g. "253-254"), or one of the following protocol names: "tcp", "udp", "icmp", "esp", "ah", "ipip", or "sctp".
    #[builder(into, default)]
    #[serde(rename = "ipProtocols")]
    pub r#ip_protocols: Box<Option<Vec<String>>>,
    /// BGP Autonomous System Number associated with the source IP address.
    #[builder(into, default)]
    #[serde(rename = "srcAsns")]
    pub r#src_asns: Box<Option<Vec<i32>>>,
    /// Source IPv4/IPv6 addresses or CIDR prefixes, in standard text format.
    #[builder(into, default)]
    #[serde(rename = "srcIpRanges")]
    pub r#src_ip_ranges: Box<Option<Vec<String>>>,
    /// Source port numbers for TCP/UDP/SCTP. Each element can be a 16-bit unsigned decimal number (e.g. "80") or range (e.g. "0-1023").
    #[builder(into, default)]
    #[serde(rename = "srcPorts")]
    pub r#src_ports: Box<Option<Vec<String>>>,
    /// Two-letter ISO 3166-1 alpha-2 country code associated with the source IP address.
    #[builder(into, default)]
    #[serde(rename = "srcRegionCodes")]
    pub r#src_region_codes: Box<Option<Vec<String>>>,
    /// User-defined fields. Each element names a defined field and lists the matching values for that field.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "userDefinedFields")]
    pub r#user_defined_fields: Box<Option<Vec<super::super::types::compute::RegionSecurityPolicyRuleNetworkMatchUserDefinedField>>>,
}
