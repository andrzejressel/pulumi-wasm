#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct OrganizationSecurityPolicyRuleMatchConfig {
    /// Destination IP address range in CIDR format. Required for
    /// EGRESS rules.
    #[builder(into, default)]
    #[serde(rename = "destIpRanges")]
    pub r#dest_ip_ranges: Box<Option<Vec<String>>>,
    /// Pairs of IP protocols and ports that the rule should match.
    /// Structure is documented below.
    /// 
    /// 
    /// <a name="nested_layer4_config"></a>The `layer4_config` block supports:
    #[builder(into)]
    #[serde(rename = "layer4Configs")]
    pub r#layer_4_configs: Box<Vec<super::super::types::compute::OrganizationSecurityPolicyRuleMatchConfigLayer4Config>>,
    /// Source IP address range in CIDR format. Required for
    /// INGRESS rules.
    #[builder(into, default)]
    #[serde(rename = "srcIpRanges")]
    pub r#src_ip_ranges: Box<Option<Vec<String>>>,
}
