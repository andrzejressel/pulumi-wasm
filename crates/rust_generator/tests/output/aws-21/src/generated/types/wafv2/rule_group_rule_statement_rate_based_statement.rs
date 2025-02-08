#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct RuleGroupRuleStatementRateBasedStatement {
    /// Setting that indicates how to aggregate the request counts. Valid values include: `CONSTANT`, `CUSTOM_KEYS`, `FORWARDED_IP` or `IP`. Default: `IP`.
    #[builder(into, default)]
    #[serde(rename = "aggregateKeyType")]
    pub r#aggregate_key_type: Box<Option<String>>,
    /// Aggregate the request counts using one or more web request components as the aggregate keys. See `custom_key` below for details.
    #[builder(into, default)]
    #[serde(rename = "customKeys")]
    pub r#custom_keys: Box<Option<Vec<super::super::types::wafv2::RuleGroupRuleStatementRateBasedStatementCustomKey>>>,
    /// The amount of time, in seconds, that AWS WAF should include in its request counts, looking back from the current time. Valid values are `60`, `120`, `300`, and `600`. Defaults to `300` (5 minutes).
    /// 
    /// **NOTE:** This setting doesn't determine how often AWS WAF checks the rate, but how far back it looks each time it checks. AWS WAF checks the rate about every 10 seconds.
    #[builder(into, default)]
    #[serde(rename = "evaluationWindowSec")]
    pub r#evaluation_window_sec: Box<Option<i32>>,
    /// The configuration for inspecting IP addresses in an HTTP header that you specify, instead of using the IP address that's reported by the web request origin. If `aggregate_key_type` is set to `FORWARDED_IP`, this block is required. See Forwarded IP Config below for details.
    #[builder(into, default)]
    #[serde(rename = "forwardedIpConfig")]
    pub r#forwarded_ip_config: Box<Option<super::super::types::wafv2::RuleGroupRuleStatementRateBasedStatementForwardedIpConfig>>,
    /// The limit on requests per 5-minute period for a single originating IP address.
    #[builder(into)]
    #[serde(rename = "limit")]
    pub r#limit: Box<i32>,
    /// An optional nested statement that narrows the scope of the rate-based statement to matching web requests. This can be any nestable statement, and you can nest statements at any level below this scope-down statement. See Statement above for details. If `aggregate_key_type` is set to `CONSTANT`, this block is required.
    #[builder(into, default)]
    #[serde(rename = "scopeDownStatement")]
    pub r#scope_down_statement: Box<Option<super::super::types::wafv2::RuleGroupRuleStatementRateBasedStatementScopeDownStatement>>,
}
