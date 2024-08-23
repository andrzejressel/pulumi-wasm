#[derive(serde::Serialize)]
pub struct TeamsRuleRuleSettings {
    #[serde(rename = "addHeaders")]
    pub r#add_headers: Box<Option<std::collections::HashMap<String, String>>>,
    #[serde(rename = "allowChildBypass")]
    pub r#allow_child_bypass: Box<Option<bool>>,
    #[serde(rename = "auditSsh")]
    pub r#audit_ssh: Box<Option<crate::types::TeamsRuleRuleSettingsAuditSsh>>,
    #[serde(rename = "bisoAdminControls")]
    pub r#biso_admin_controls: Box<Option<crate::types::TeamsRuleRuleSettingsBisoAdminControls>>,
    #[serde(rename = "blockPageEnabled")]
    pub r#block_page_enabled: Box<Option<bool>>,
    #[serde(rename = "blockPageReason")]
    pub r#block_page_reason: Box<Option<String>>,
    #[serde(rename = "bypassParentRule")]
    pub r#bypass_parent_rule: Box<Option<bool>>,
    #[serde(rename = "checkSession")]
    pub r#check_session: Box<Option<crate::types::TeamsRuleRuleSettingsCheckSession>>,
    #[serde(rename = "egress")]
    pub r#egress: Box<Option<crate::types::TeamsRuleRuleSettingsEgress>>,
    #[serde(rename = "insecureDisableDnssecValidation")]
    pub r#insecure_disable_dnssec_validation: Box<Option<bool>>,
    #[serde(rename = "ipCategories")]
    pub r#ip_categories: Box<Option<bool>>,
    #[serde(rename = "l4override")]
    pub r#l_4_override: Box<Option<crate::types::TeamsRuleRuleSettingsL4Override>>,
    #[serde(rename = "notificationSettings")]
    pub r#notification_settings:
        Box<Option<crate::types::TeamsRuleRuleSettingsNotificationSettings>>,
    #[serde(rename = "overrideHost")]
    pub r#override_host: Box<Option<String>>,
    #[serde(rename = "overrideIps")]
    pub r#override_ips: Box<Option<Vec<String>>>,
    #[serde(rename = "payloadLog")]
    pub r#payload_log: Box<Option<crate::types::TeamsRuleRuleSettingsPayloadLog>>,
    #[serde(rename = "untrustedCert")]
    pub r#untrusted_cert: Box<Option<crate::types::TeamsRuleRuleSettingsUntrustedCert>>,
}
