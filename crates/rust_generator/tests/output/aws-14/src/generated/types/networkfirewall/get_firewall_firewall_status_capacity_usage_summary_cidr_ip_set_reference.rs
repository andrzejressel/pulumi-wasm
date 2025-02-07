#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetFirewallFirewallStatusCapacityUsageSummaryCidrIpSetReference {
    /// Total number of CIDR blocks used by the IP set references in a firewall.
    #[builder(into)]
    #[serde(rename = "resolvedCidrCount")]
    pub r#resolved_cidr_count: Box<i32>,
}
