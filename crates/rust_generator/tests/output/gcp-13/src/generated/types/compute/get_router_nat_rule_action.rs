#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct GetRouterNatRuleAction {
    /// A list of URLs of the IP resources used for this NAT rule.
    /// These IP addresses must be valid static external IP addresses assigned to the project.
    /// This field is used for public NAT.
    #[builder(into)]
    #[serde(rename = "sourceNatActiveIps")]
    pub r#source_nat_active_ips: Box<Vec<String>>,
    /// A list of URLs of the subnetworks used as source ranges for this NAT Rule.
    /// These subnetworks must have purpose set to PRIVATE_NAT.
    /// This field is used for private NAT.
    #[builder(into)]
    #[serde(rename = "sourceNatActiveRanges")]
    pub r#source_nat_active_ranges: Box<Vec<String>>,
    /// A list of URLs of the IP resources to be drained.
    /// These IPs must be valid static external IPs that have been assigned to the NAT.
    /// These IPs should be used for updating/patching a NAT rule only.
    /// This field is used for public NAT.
    #[builder(into)]
    #[serde(rename = "sourceNatDrainIps")]
    pub r#source_nat_drain_ips: Box<Vec<String>>,
    /// A list of URLs of subnetworks representing source ranges to be drained.
    /// This is only supported on patch/update, and these subnetworks must have previously been used as active ranges in this NAT Rule.
    /// This field is used for private NAT.
    #[builder(into)]
    #[serde(rename = "sourceNatDrainRanges")]
    pub r#source_nat_drain_ranges: Box<Vec<String>>,
}
