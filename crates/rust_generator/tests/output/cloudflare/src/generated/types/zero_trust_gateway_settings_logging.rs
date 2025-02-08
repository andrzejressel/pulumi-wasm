#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct ZeroTrustGatewaySettingsLogging {
    /// Redact personally identifiable information from activity logging (PII fields are: source IP, user email, user ID, device ID, URL, referrer, user agent).
    #[builder(into)]
    #[serde(rename = "redactPii")]
    pub r#redact_pii: Box<bool>,
    /// Represents whether all requests are logged or only the blocked requests are slogged in DNS, HTTP and L4 filters.
    #[builder(into)]
    #[serde(rename = "settingsByRuleType")]
    pub r#settings_by_rule_type: Box<super::types::ZeroTrustGatewaySettingsLoggingSettingsByRuleType>,
}
