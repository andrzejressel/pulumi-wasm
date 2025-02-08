#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct FirewallPolicyDns {
    /// Whether to enable DNS proxy on Firewalls attached to this Firewall Policy? Defaults to `false`.
    #[builder(into, default)]
    #[serde(rename = "proxyEnabled")]
    pub r#proxy_enabled: Box<Option<bool>>,
    /// A list of custom DNS servers' IP addresses.
    #[builder(into, default)]
    #[serde(rename = "servers")]
    pub r#servers: Box<Option<Vec<String>>>,
}
