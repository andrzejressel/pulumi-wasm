#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct FirewallPolicyWithRulesPredefinedRuleMatch {
    /// Address groups which should be matched against the traffic destination.
    /// Maximum number of destination address groups is 10.
    #[builder(into, default)]
    #[serde(rename = "destAddressGroups")]
    pub r#dest_address_groups: Box<Option<Vec<String>>>,
    /// Fully Qualified Domain Name (FQDN) which should be matched against
    /// traffic destination. Maximum number of destination fqdn allowed is 100.
    #[builder(into, default)]
    #[serde(rename = "destFqdns")]
    pub r#dest_fqdns: Box<Option<Vec<String>>>,
    /// Destination IP address range in CIDR format. Required for
    /// EGRESS rules.
    #[builder(into, default)]
    #[serde(rename = "destIpRanges")]
    pub r#dest_ip_ranges: Box<Option<Vec<String>>>,
    /// Region codes whose IP addresses will be used to match for destination
    /// of traffic. Should be specified as 2 letter country code defined as per
    /// ISO 3166 alpha-2 country codes. ex."US"
    /// Maximum number of destination region codes allowed is 5000.
    #[builder(into, default)]
    #[serde(rename = "destRegionCodes")]
    pub r#dest_region_codes: Box<Option<Vec<String>>>,
    /// Names of Network Threat Intelligence lists.
    /// The IPs in these lists will be matched against traffic destination.
    #[builder(into, default)]
    #[serde(rename = "destThreatIntelligences")]
    pub r#dest_threat_intelligences: Box<Option<Vec<String>>>,
    /// Pairs of IP protocols and ports that the rule should match.
    /// Structure is documented below.
    /// 
    /// 
    /// <a name="nested_layer4_config"></a>The `layer4_config` block supports:
    #[builder(into, default)]
    #[serde(rename = "layer4Configs")]
    pub r#layer_4_configs: Box<Option<Vec<super::super::types::compute::FirewallPolicyWithRulesPredefinedRuleMatchLayer4Config>>>,
    /// Address groups which should be matched against the traffic source.
    /// Maximum number of source address groups is 10.
    #[builder(into, default)]
    #[serde(rename = "srcAddressGroups")]
    pub r#src_address_groups: Box<Option<Vec<String>>>,
    /// Fully Qualified Domain Name (FQDN) which should be matched against
    /// traffic source. Maximum number of source fqdn allowed is 100.
    #[builder(into, default)]
    #[serde(rename = "srcFqdns")]
    pub r#src_fqdns: Box<Option<Vec<String>>>,
    /// Source IP address range in CIDR format. Required for
    /// INGRESS rules.
    #[builder(into, default)]
    #[serde(rename = "srcIpRanges")]
    pub r#src_ip_ranges: Box<Option<Vec<String>>>,
    /// Region codes whose IP addresses will be used to match for source
    /// of traffic. Should be specified as 2 letter country code defined as per
    /// ISO 3166 alpha-2 country codes. ex."US"
    /// Maximum number of source region codes allowed is 5000.
    #[builder(into, default)]
    #[serde(rename = "srcRegionCodes")]
    pub r#src_region_codes: Box<Option<Vec<String>>>,
    /// Names of Network Threat Intelligence lists.
    /// The IPs in these lists will be matched against traffic source.
    #[builder(into, default)]
    #[serde(rename = "srcThreatIntelligences")]
    pub r#src_threat_intelligences: Box<Option<Vec<String>>>,
}
