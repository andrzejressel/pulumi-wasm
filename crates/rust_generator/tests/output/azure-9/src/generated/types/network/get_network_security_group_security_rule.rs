#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetNetworkSecurityGroupSecurityRule {
    /// Is network traffic is allowed or denied?
    #[builder(into)]
    #[serde(rename = "access")]
    pub r#access: Box<String>,
    /// The description for this rule.
    #[builder(into)]
    #[serde(rename = "description")]
    pub r#description: Box<String>,
    /// CIDR or destination IP range or * to match any IP.
    #[builder(into)]
    #[serde(rename = "destinationAddressPrefix")]
    pub r#destination_address_prefix: Box<String>,
    /// A list of CIDRs or destination IP ranges.
    #[builder(into)]
    #[serde(rename = "destinationAddressPrefixes")]
    pub r#destination_address_prefixes: Box<Vec<String>>,
    /// A List of destination Application Security Group IDs
    #[builder(into, default)]
    #[serde(rename = "destinationApplicationSecurityGroupIds")]
    pub r#destination_application_security_group_ids: Box<Option<Vec<String>>>,
    /// The Destination Port or Range.
    #[builder(into)]
    #[serde(rename = "destinationPortRange")]
    pub r#destination_port_range: Box<String>,
    #[builder(into)]
    #[serde(rename = "destinationPortRanges")]
    pub r#destination_port_ranges: Box<Vec<String>>,
    /// The direction specifies if rule will be evaluated on incoming or outgoing traffic.
    #[builder(into)]
    #[serde(rename = "direction")]
    pub r#direction: Box<String>,
    /// Specifies the Name of the Network Security Group.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// The priority of the rule
    #[builder(into)]
    #[serde(rename = "priority")]
    pub r#priority: Box<i32>,
    /// The network protocol this rule applies to.
    #[builder(into)]
    #[serde(rename = "protocol")]
    pub r#protocol: Box<String>,
    /// CIDR or source IP range or * to match any IP.
    #[builder(into)]
    #[serde(rename = "sourceAddressPrefix")]
    pub r#source_address_prefix: Box<String>,
    /// A list of CIDRs or source IP ranges.
    #[builder(into)]
    #[serde(rename = "sourceAddressPrefixes")]
    pub r#source_address_prefixes: Box<Vec<String>>,
    /// A List of source Application Security Group IDs
    #[builder(into, default)]
    #[serde(rename = "sourceApplicationSecurityGroupIds")]
    pub r#source_application_security_group_ids: Box<Option<Vec<String>>>,
    /// The Source Port or Range.
    #[builder(into)]
    #[serde(rename = "sourcePortRange")]
    pub r#source_port_range: Box<String>,
    #[builder(into)]
    #[serde(rename = "sourcePortRanges")]
    pub r#source_port_ranges: Box<Vec<String>>,
}
