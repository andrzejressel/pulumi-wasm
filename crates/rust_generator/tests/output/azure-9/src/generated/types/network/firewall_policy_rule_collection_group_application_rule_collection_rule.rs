#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct FirewallPolicyRuleCollectionGroupApplicationRuleCollectionRule {
    #[builder(into, default)]
    #[serde(rename = "description")]
    pub r#description: Box<Option<String>>,
    #[builder(into, default)]
    #[serde(rename = "destinationAddresses")]
    pub r#destination_addresses: Box<Option<Vec<String>>>,
    /// Specifies a list of destination FQDN tags.
    #[builder(into, default)]
    #[serde(rename = "destinationFqdnTags")]
    pub r#destination_fqdn_tags: Box<Option<Vec<String>>>,
    #[builder(into, default)]
    #[serde(rename = "destinationFqdns")]
    pub r#destination_fqdns: Box<Option<Vec<String>>>,
    /// Specifies a list of destination URLs for which policy should hold. Needs Premium SKU for Firewall Policy. Conflicts with `destination_fqdns`.
    #[builder(into, default)]
    #[serde(rename = "destinationUrls")]
    pub r#destination_urls: Box<Option<Vec<String>>>,
    /// Specifies a list of HTTP/HTTPS headers to insert. One or more `http_headers` blocks as defined below.
    #[builder(into, default)]
    #[serde(rename = "httpHeaders")]
    pub r#http_headers: Box<Option<Vec<super::super::types::network::FirewallPolicyRuleCollectionGroupApplicationRuleCollectionRuleHttpHeader>>>,
    /// The name which should be used for this Firewall Policy Rule Collection Group. Changing this forces a new Firewall Policy Rule Collection Group to be created.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    #[builder(into, default)]
    #[serde(rename = "protocols")]
    pub r#protocols: Box<Option<Vec<super::super::types::network::FirewallPolicyRuleCollectionGroupApplicationRuleCollectionRuleProtocol>>>,
    #[builder(into, default)]
    #[serde(rename = "sourceAddresses")]
    pub r#source_addresses: Box<Option<Vec<String>>>,
    #[builder(into, default)]
    #[serde(rename = "sourceIpGroups")]
    pub r#source_ip_groups: Box<Option<Vec<String>>>,
    /// Boolean specifying if TLS shall be terminated (true) or not (false). Must be `true` when using `destination_urls`. Needs Premium SKU for Firewall Policy.
    #[builder(into, default)]
    #[serde(rename = "terminateTls")]
    pub r#terminate_tls: Box<Option<bool>>,
    /// Specifies a list of web categories to which access is denied or allowed depending on the value of `action` above. Needs Premium SKU for Firewall Policy.
    #[builder(into, default)]
    #[serde(rename = "webCategories")]
    pub r#web_categories: Box<Option<Vec<String>>>,
}
