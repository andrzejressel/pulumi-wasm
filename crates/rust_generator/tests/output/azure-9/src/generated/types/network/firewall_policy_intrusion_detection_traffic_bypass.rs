#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct FirewallPolicyIntrusionDetectionTrafficBypass {
    /// The description for this bypass traffic setting.
    #[builder(into, default)]
    #[serde(rename = "description")]
    pub r#description: Box<Option<String>>,
    /// Specifies a list of destination IP addresses that shall be bypassed by intrusion detection.
    #[builder(into, default)]
    #[serde(rename = "destinationAddresses")]
    pub r#destination_addresses: Box<Option<Vec<String>>>,
    /// Specifies a list of destination IP groups that shall be bypassed by intrusion detection.
    #[builder(into, default)]
    #[serde(rename = "destinationIpGroups")]
    pub r#destination_ip_groups: Box<Option<Vec<String>>>,
    /// Specifies a list of destination IP ports that shall be bypassed by intrusion detection.
    #[builder(into, default)]
    #[serde(rename = "destinationPorts")]
    pub r#destination_ports: Box<Option<Vec<String>>>,
    /// The name which should be used for this bypass traffic setting.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// The protocols any of `ANY`, `TCP`, `ICMP`, `UDP` that shall be bypassed by intrusion detection.
    #[builder(into)]
    #[serde(rename = "protocol")]
    pub r#protocol: Box<String>,
    /// Specifies a list of source addresses that shall be bypassed by intrusion detection.
    #[builder(into, default)]
    #[serde(rename = "sourceAddresses")]
    pub r#source_addresses: Box<Option<Vec<String>>>,
    /// Specifies a list of source IP groups that shall be bypassed by intrusion detection.
    #[builder(into, default)]
    #[serde(rename = "sourceIpGroups")]
    pub r#source_ip_groups: Box<Option<Vec<String>>>,
}
