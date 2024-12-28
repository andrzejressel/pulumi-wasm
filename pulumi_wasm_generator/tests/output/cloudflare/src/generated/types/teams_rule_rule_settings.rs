#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct TeamsRuleRuleSettings {
    /// Add custom headers to allowed requests in the form of key-value pairs.
    #[builder(into, default)]
    #[serde(rename = "addHeaders")]
    pub r#add_headers: Box<Option<std::collections::HashMap<String, String>>>,
    /// Allow parent MSP accounts to enable bypass their children's rules.
    #[builder(into, default)]
    #[serde(rename = "allowChildBypass")]
    pub r#allow_child_bypass: Box<Option<bool>>,
    /// Settings for auditing SSH usage.
    #[builder(into, default)]
    #[serde(rename = "auditSsh")]
    pub r#audit_ssh: Box<Option<super::types::TeamsRuleRuleSettingsAuditSsh>>,
    /// Configure how browser isolation behaves.
    #[builder(into, default)]
    #[serde(rename = "bisoAdminControls")]
    pub r#biso_admin_controls: Box<Option<super::types::TeamsRuleRuleSettingsBisoAdminControls>>,
    /// Indicator of block page enablement.
    #[builder(into, default)]
    #[serde(rename = "blockPageEnabled")]
    pub r#block_page_enabled: Box<Option<bool>>,
    /// The displayed reason for a user being blocked.
    #[builder(into, default)]
    #[serde(rename = "blockPageReason")]
    pub r#block_page_reason: Box<Option<String>>,
    /// Allow child MSP accounts to bypass their parent's rule.
    #[builder(into, default)]
    #[serde(rename = "bypassParentRule")]
    pub r#bypass_parent_rule: Box<Option<bool>>,
    /// Configure how session check behaves.
    #[builder(into, default)]
    #[serde(rename = "checkSession")]
    pub r#check_session: Box<Option<super::types::TeamsRuleRuleSettingsCheckSession>>,
    /// Add your own custom resolvers to route queries that match the resolver policy. Cannot be used when resolve*dns*through*cloudflare is set. DNS queries will route to the address closest to their origin.
    #[builder(into, default)]
    #[serde(rename = "dnsResolvers")]
    pub r#dns_resolvers: Box<Option<super::types::TeamsRuleRuleSettingsDnsResolvers>>,
    /// Configure how Proxy traffic egresses. Can be set for rules with Egress action and Egress filter. Can be omitted to indicate local egress via Warp IPs.
    #[builder(into, default)]
    #[serde(rename = "egress")]
    pub r#egress: Box<Option<super::types::TeamsRuleRuleSettingsEgress>>,
    /// Set to true, to ignore the category matches at CNAME domains in a response.
    #[builder(into, default)]
    #[serde(rename = "ignoreCnameCategoryMatches")]
    pub r#ignore_cname_category_matches: Box<Option<bool>>,
    /// Disable DNSSEC validation (must be Allow rule).
    #[builder(into, default)]
    #[serde(rename = "insecureDisableDnssecValidation")]
    pub r#insecure_disable_dnssec_validation: Box<Option<bool>>,
    /// Turns on IP category based filter on dns if the rule contains dns category checks.
    #[builder(into, default)]
    #[serde(rename = "ipCategories")]
    pub r#ip_categories: Box<Option<bool>>,
    /// Settings to forward layer 4 traffic.
    #[builder(into, default)]
    #[serde(rename = "l4override")]
    pub r#l_4_override: Box<Option<super::types::TeamsRuleRuleSettingsL4Override>>,
    /// Notification settings on a block rule.
    #[builder(into, default)]
    #[serde(rename = "notificationSettings")]
    pub r#notification_settings: Box<Option<super::types::TeamsRuleRuleSettingsNotificationSettings>>,
    /// The host to override matching DNS queries with.
    #[builder(into, default)]
    #[serde(rename = "overrideHost")]
    pub r#override_host: Box<Option<String>>,
    /// The IPs to override matching DNS queries with.
    #[builder(into, default)]
    #[serde(rename = "overrideIps")]
    pub r#override_ips: Box<Option<Vec<String>>>,
    /// Configure DLP Payload Logging settings for this rule.
    #[builder(into, default)]
    #[serde(rename = "payloadLog")]
    pub r#payload_log: Box<Option<super::types::TeamsRuleRuleSettingsPayloadLog>>,
    /// Enable sending queries that match the resolver policy to Cloudflare's default 1.1.1.1 DNS resolver. Cannot be set when `dns_resolvers` are specified.
    #[builder(into, default)]
    #[serde(rename = "resolveDnsThroughCloudflare")]
    pub r#resolve_dns_through_cloudflare: Box<Option<bool>>,
    /// Configure untrusted certificate settings for this rule.
    #[builder(into, default)]
    #[serde(rename = "untrustedCert")]
    pub r#untrusted_cert: Box<Option<super::types::TeamsRuleRuleSettingsUntrustedCert>>,
}
