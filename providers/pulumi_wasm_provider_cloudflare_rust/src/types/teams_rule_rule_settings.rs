#[derive(serde::Serialize)]
pub struct TeamsRuleRuleSettings {
    /// Add custom headers to allowed requests in the form of key-value pairs.
    #[serde(rename = "addHeaders")]
    pub r#add_headers: Box<Option<std::collections::HashMap<String, String>>>,
    /// Allow parent MSP accounts to enable bypass their children's rules.
    #[serde(rename = "allowChildBypass")]
    pub r#allow_child_bypass: Box<Option<bool>>,
    /// Settings for auditing SSH usage.
    #[serde(rename = "auditSsh")]
    pub r#audit_ssh: Box<Option<crate::types::TeamsRuleRuleSettingsAuditSsh>>,
    /// Configure how browser isolation behaves.
    #[serde(rename = "bisoAdminControls")]
    pub r#biso_admin_controls: Box<Option<crate::types::TeamsRuleRuleSettingsBisoAdminControls>>,
    /// Indicator of block page enablement.
    #[serde(rename = "blockPageEnabled")]
    pub r#block_page_enabled: Box<Option<bool>>,
    /// The displayed reason for a user being blocked.
    #[serde(rename = "blockPageReason")]
    pub r#block_page_reason: Box<Option<String>>,
    /// Allow child MSP accounts to bypass their parent's rule.
    #[serde(rename = "bypassParentRule")]
    pub r#bypass_parent_rule: Box<Option<bool>>,
    /// Configure how session check behaves.
    #[serde(rename = "checkSession")]
    pub r#check_session: Box<Option<crate::types::TeamsRuleRuleSettingsCheckSession>>,
    /// Configure how Proxy traffic egresses. Can be set for rules with Egress action and Egress filter. Can be omitted to indicate local egress via Warp IPs.
    #[serde(rename = "egress")]
    pub r#egress: Box<Option<crate::types::TeamsRuleRuleSettingsEgress>>,
    /// Disable DNSSEC validation (must be Allow rule).
    #[serde(rename = "insecureDisableDnssecValidation")]
    pub r#insecure_disable_dnssec_validation: Box<Option<bool>>,
    /// Turns on IP category based filter on dns if the rule contains dns category checks.
    #[serde(rename = "ipCategories")]
    pub r#ip_categories: Box<Option<bool>>,
    /// Settings to forward layer 4 traffic.
    #[serde(rename = "l4override")]
    pub r#l_4_override: Box<Option<crate::types::TeamsRuleRuleSettingsL4Override>>,
    /// Notification settings on a block rule.
    #[serde(rename = "notificationSettings")]
    pub r#notification_settings: Box<Option<crate::types::TeamsRuleRuleSettingsNotificationSettings>>,
    /// The host to override matching DNS queries with.
    #[serde(rename = "overrideHost")]
    pub r#override_host: Box<Option<String>>,
    /// The IPs to override matching DNS queries with.
    #[serde(rename = "overrideIps")]
    pub r#override_ips: Box<Option<Vec<String>>>,
    /// Configure DLP Payload Logging settings for this rule.
    #[serde(rename = "payloadLog")]
    pub r#payload_log: Box<Option<crate::types::TeamsRuleRuleSettingsPayloadLog>>,
    /// Configure untrusted certificate settings for this rule.
    #[serde(rename = "untrustedCert")]
    pub r#untrusted_cert: Box<Option<crate::types::TeamsRuleRuleSettingsUntrustedCert>>,
}
