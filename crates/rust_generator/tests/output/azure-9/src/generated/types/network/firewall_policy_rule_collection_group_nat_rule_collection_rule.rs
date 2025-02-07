#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct FirewallPolicyRuleCollectionGroupNatRuleCollectionRule {
    #[builder(into, default)]
    #[serde(rename = "description")]
    pub r#description: Box<Option<String>>,
    /// The destination IP address (including CIDR).
    #[builder(into, default)]
    #[serde(rename = "destinationAddress")]
    pub r#destination_address: Box<Option<String>>,
    #[builder(into, default)]
    #[serde(rename = "destinationPorts")]
    pub r#destination_ports: Box<Option<String>>,
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
    /// Specifies the translated address.
    #[builder(into, default)]
    #[serde(rename = "translatedAddress")]
    pub r#translated_address: Box<Option<String>>,
    /// Specifies the translated FQDN.
    /// 
    /// > **NOTE:** Exactly one of `translated_address` and `translated_fqdn` should be set.
    #[builder(into, default)]
    #[serde(rename = "translatedFqdn")]
    pub r#translated_fqdn: Box<Option<String>>,
    /// Specifies the translated port.
    #[builder(into)]
    #[serde(rename = "translatedPort")]
    pub r#translated_port: Box<i32>,
}
