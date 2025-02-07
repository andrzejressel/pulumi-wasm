#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct RouterNatSubnetwork {
    /// Self-link of subnetwork to NAT
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// List of the secondary ranges of the subnetwork that are allowed
    /// to use NAT. This can be populated only if
    /// `LIST_OF_SECONDARY_IP_RANGES` is one of the values in
    /// sourceIpRangesToNat
    #[builder(into, default)]
    #[serde(rename = "secondaryIpRangeNames")]
    pub r#secondary_ip_range_names: Box<Option<Vec<String>>>,
    /// List of options for which source IPs in the subnetwork
    /// should have NAT enabled. Supported values include:
    /// `ALL_IP_RANGES`, `LIST_OF_SECONDARY_IP_RANGES`,
    /// `PRIMARY_IP_RANGE`.
    #[builder(into)]
    #[serde(rename = "sourceIpRangesToNats")]
    pub r#source_ip_ranges_to_nats: Box<Vec<String>>,
}
