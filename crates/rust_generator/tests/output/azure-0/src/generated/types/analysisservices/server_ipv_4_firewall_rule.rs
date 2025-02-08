#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ServerIpv4FirewallRule {
    /// Specifies the name of the firewall rule.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// End of the firewall rule range as IPv4 address.
    #[builder(into)]
    #[serde(rename = "rangeEnd")]
    pub r#range_end: Box<String>,
    /// Start of the firewall rule range as IPv4 address.
    #[builder(into)]
    #[serde(rename = "rangeStart")]
    pub r#range_start: Box<String>,
}
