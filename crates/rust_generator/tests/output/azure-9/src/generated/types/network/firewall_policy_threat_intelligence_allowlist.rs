#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct FirewallPolicyThreatIntelligenceAllowlist {
    /// A list of FQDNs that will be skipped for threat detection.
    #[builder(into, default)]
    #[serde(rename = "fqdns")]
    pub r#fqdns: Box<Option<Vec<String>>>,
    /// A list of IP addresses or CIDR ranges that will be skipped for threat detection.
    #[builder(into, default)]
    #[serde(rename = "ipAddresses")]
    pub r#ip_addresses: Box<Option<Vec<String>>>,
}
