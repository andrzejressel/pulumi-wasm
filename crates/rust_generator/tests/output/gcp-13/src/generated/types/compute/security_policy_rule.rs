#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct SecurityPolicyRule {
    /// Action to take when `match` matches the request. Valid values:
    #[builder(into)]
    #[serde(rename = "action")]
    pub r#action: Box<String>,
    /// An optional description of this rule. Max size is 64.
    #[builder(into, default)]
    #[serde(rename = "description")]
    pub r#description: Box<Option<String>>,
    /// Additional actions that are performed on headers. Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "headerAction")]
    pub r#header_action: Box<Option<super::super::types::compute::SecurityPolicyRuleHeaderAction>>,
    /// A match condition that incoming traffic is evaluated against.
    /// If it evaluates to true, the corresponding `action` is enforced. Structure is documented below.
    #[builder(into)]
    #[serde(rename = "match")]
    pub r#match_: Box<super::super::types::compute::SecurityPolicyRuleMatch>,
    /// Preconfigured WAF configuration to be applied for the rule. If the rule does not evaluate preconfigured WAF rules, i.e., if `evaluatePreconfiguredWaf()` is not used, this field will have no effect. Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "preconfiguredWafConfig")]
    pub r#preconfigured_waf_config: Box<Option<super::super::types::compute::SecurityPolicyRulePreconfiguredWafConfig>>,
    /// When set to true, the `action` specified above is not enforced.
    /// Stackdriver logs for requests that trigger a preview action are annotated as such.
    #[builder(into, default)]
    #[serde(rename = "preview")]
    pub r#preview: Box<Option<bool>>,
    /// An unique positive integer indicating the priority of evaluation for a rule.
    /// Rules are evaluated from highest priority (lowest numerically) to lowest priority (highest numerically) in order.
    #[builder(into)]
    #[serde(rename = "priority")]
    pub r#priority: Box<i32>,
    /// Must be specified if the `action` is `rate_based_ban` or `throttle`. Cannot be specified for other actions. Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "rateLimitOptions")]
    pub r#rate_limit_options: Box<Option<super::super::types::compute::SecurityPolicyRuleRateLimitOptions>>,
    /// Can be specified if the `action` is `redirect`. Cannot be specified for other actions. Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "redirectOptions")]
    pub r#redirect_options: Box<Option<super::super::types::compute::SecurityPolicyRuleRedirectOptions>>,
}
