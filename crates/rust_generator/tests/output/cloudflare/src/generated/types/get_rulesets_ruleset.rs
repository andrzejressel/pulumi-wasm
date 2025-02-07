#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetRulesetsRuleset {
    /// Brief summary of the ruleset and its intended use.
    #[builder(into, default)]
    #[serde(rename = "description")]
    pub r#description: Box<Option<String>>,
    /// ID of the ruleset.
    #[builder(into)]
    #[serde(rename = "id")]
    pub r#id: Box<String>,
    /// Type of Ruleset. Available values: `custom`, `managed`, `root`, `zone`
    #[builder(into)]
    #[serde(rename = "kind")]
    pub r#kind: Box<String>,
    /// Name of the ruleset.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// Point in the request/response lifecycle where the ruleset executes. Available values: `ddos_l4`, `ddos_l7`, `http_config_settings`, `http_custom_errors`, `http_log_custom_fields`, `http_ratelimit`, `http_request_cache_settings`, `http_request_dynamic_redirect`, `http_request_firewall_custom`, `http_request_firewall_managed`, `http_request_late_transform`, `http_request_origin`, `http_request_redirect`, `http_request_sanitize`, `http_request_sbfm`, `http_request_transform`, `http_response_compression`, `http_response_firewall_managed`, `http_response_headers_transform`, `magic_transit`
    #[builder(into)]
    #[serde(rename = "phase")]
    pub r#phase: Box<String>,
    /// List of rules to apply to the ruleset.
    #[builder(into, default)]
    #[serde(rename = "rules")]
    pub r#rules: Box<Option<Vec<super::types::GetRulesetsRulesetRule>>>,
    /// Version of the ruleset.
    #[builder(into)]
    #[serde(rename = "version")]
    pub r#version: Box<String>,
}
