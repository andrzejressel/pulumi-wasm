#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct FirewallPolicyRuleCollectionGroupNetworkRuleCollectionRule {
    #[builder(into, default)]
    #[serde(rename = "description")]
    pub r#description: Box<Option<String>>,
    #[builder(into, default)]
    #[serde(rename = "destinationAddresses")]
    pub r#destination_addresses: Box<Option<Vec<String>>>,
    #[builder(into, default)]
    #[serde(rename = "destinationFqdns")]
    pub r#destination_fqdns: Box<Option<Vec<String>>>,
    /// Specifies a list of destination IP groups.
    #[builder(into, default)]
    #[serde(rename = "destinationIpGroups")]
    pub r#destination_ip_groups: Box<Option<Vec<String>>>,
    #[builder(into)]
    #[serde(rename = "destinationPorts")]
    pub r#destination_ports: Box<Vec<String>>,
    /// The name which should be used for this Firewall Policy Rule Collection Group. Changing this forces a new Firewall Policy Rule Collection Group to be created.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    #[builder(into)]
    #[serde(rename = "protocols")]
    pub r#protocols: Box<Vec<String>>,
    #[builder(into, default)]
    #[serde(rename = "sourceAddresses")]
    pub r#source_addresses: Box<Option<Vec<String>>>,
    #[builder(into, default)]
    #[serde(rename = "sourceIpGroups")]
    pub r#source_ip_groups: Box<Option<Vec<String>>>,
}
